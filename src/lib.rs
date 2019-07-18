use wasm_bindgen::prelude::*;
use js_sys::{Array, Reflect, Boolean, Set, Object};
use std::rc::Rc;

mod debugging;

pub use https_everywhere_lib_core::{Rule, CookieRule, RuleSet};
use https_everywhere_lib_core::RuleSets;

const ERR: &str = "could not convert property to JS";

#[derive(Debug)]
struct StaticJsStrings {
    active: JsValue,
    cookierules: JsValue,
    default_off: JsValue,
    default_state: JsValue,
    exclusion: JsValue,
    exclusions: JsValue,
    from: JsValue,
    host: JsValue,
    mixed_content: JsValue,
    name: JsValue,
    note: JsValue,
    platform: JsValue,
    rule: JsValue,
    rules: JsValue,
    scope: JsValue,
    securecookie: JsValue,
    state: JsValue,
    target: JsValue,
    to: JsValue,
    user_rule: JsValue,
}

thread_local! {
    static JS_STRINGS: StaticJsStrings = StaticJsStrings {
        active: JsValue::from("active"),
        cookierules: JsValue::from("cookierules"),
        default_off: JsValue::from("default_off"),
        default_state: JsValue::from("default_state"),
        exclusion: JsValue::from("exclusion"),
        exclusions: JsValue::from("exclusions"),
        from: JsValue::from("from"),
        host: JsValue::from("host"),
        mixed_content: JsValue::from("mixedcontent"),
        name: JsValue::from("name"),
        note: JsValue::from("note"),
        platform: JsValue::from("platform"),
        rule: JsValue::from("rule"),
        rules: JsValue::from("rules"),
        scope: JsValue::from("scope"),
        securecookie: JsValue::from("securecookie"),
        state: JsValue::from("state"),
        target: JsValue::from("target"),
        to: JsValue::from("to"),
        user_rule: JsValue::from("user rule"),
    };
}

pub trait ToJavaScript {
    fn to_javascript(&self) -> JsValue;
}

impl ToJavaScript for Vec<Rc<RuleSet>>{
    /// Convert a vector of rulesets to a JS value
    fn to_javascript(&self) -> JsValue {
        let results = Set::new(&Array::new());
        for rs in self {
            results.add(&rs.to_javascript());
        }
        results.into()
    }
}

impl ToJavaScript for Rule {
    /// Convert a rule to a JS value
    fn to_javascript(&self) -> JsValue {
        let object = Object::new();
        JS_STRINGS.with(|jss| {
            match &self {
                Rule::Trivial => {
                    Reflect::set(&object, &jss.from, &JsValue::from("^http:")).expect(ERR);
                    Reflect::set(&object, &jss.to, &JsValue::from("https:")).expect(ERR);
                },
                Rule::NonTrivial(from_regex, to) => {
                    Reflect::set(&object, &jss.from, &JsValue::from(from_regex)).expect(ERR);
                    Reflect::set(&object, &jss.to, &JsValue::from(to)).expect(ERR);
                }
            }
        });
        object.into()
    }
}

impl ToJavaScript for CookieRule {
    /// Convert a ruleset to a JS value
    fn to_javascript(&self) -> JsValue {
        let object = Object::new();
        JS_STRINGS.with(|jss| {
            Reflect::set(&object, &jss.host, &JsValue::from(&self.host_regex)).expect(ERR);
            Reflect::set(&object, &jss.name, &JsValue::from(&self.name_regex)).expect(ERR);
        });
        object.into()
    }
}

impl ToJavaScript for RuleSet {
    /// Convert a ruleset to a JS object
    fn to_javascript(&self) -> JsValue {
        let object = Object::new();
        JS_STRINGS.with(|jss| {
            Reflect::set(&object, &jss.name, &JsValue::from(&self.name)).expect(ERR);
            Reflect::set(&object, &jss.active, &JsValue::from_bool(self.active.clone())).expect(ERR);
            Reflect::set(&object, &jss.default_state, &JsValue::from_bool(self.default_state.clone())).expect(ERR);
            match self.scope.as_ref() {
                Some(scope) => { Reflect::set(&object, &jss.scope, &JsValue::from(scope)).expect(ERR); },
                None => {}
            }
            match &self.note {
                Some(note) => { Reflect::set(&object, &jss.note, &JsValue::from(note)).expect(ERR); },
                None => {}
            }

            let rules = Array::new();
            for rule in &self.rules {
                rules.push(&rule.to_javascript());
            }
            Reflect::set(&object, &jss.rules, &rules).expect(ERR);

            match &self.exclusions {
                Some(exclusions) => {
                    Reflect::set(&object, &jss.exclusions, &JsValue::from(exclusions)).expect(ERR);
                },
                None => {}
            }

            match &self.cookierules {
                Some(cookierules) => {
                    let cookierules_array = Array::new();
                    for cookierule in cookierules {
                        cookierules_array.push(&cookierule.to_javascript());
                    }
                    Reflect::set(&object, &jss.cookierules, &cookierules_array).expect(ERR);
                },
                None => {}
            }
        });
        object.into()
    }
}

pub trait JsRuleSet {
    fn add_rules(&mut self, rules_array: &Array);
    fn add_exclusions(&mut self, exclusions_array: &Array);
    fn add_cookierules(&mut self, cookierules_array: &Array);
    fn is_equivalent_to(&self, ruleset_jsval: &JsValue) -> bool;
}

impl JsRuleSet for RuleSet {
    /// Add rules, specified in JS array
    fn add_rules(&mut self, rules_array: &Array){
        for rule_result in rules_array.values() {
            let rule_obj = rule_result.unwrap();
            if rule_obj.is_object() {
                JS_STRINGS.with(|jss| {
                    let from_string = match Reflect::get(&rule_obj, &jss.from) {
                        Ok(from) => {
                            if from.is_string() {
                                from.as_string().unwrap()
                            } else {
                                String::new()
                            }
                        },
                        _ => String::new()
                    };
                    let to_string = match Reflect::get(&rule_obj, &jss.to) {
                        Ok(to) => {
                            if to.is_string() {
                                to.as_string().unwrap()
                            } else {
                                String::new()
                            }
                        },
                        _ => String::new()
                    };

                    self.rules.push(Rule::new(from_string, to_string));
                });
            }
        };
    }

    /// Add exclusions to the ruleset from an exclusions JS array
    fn add_exclusions(&mut self, exclusions_array: &Array){
        let mut exclusions = vec![];
        for exclusion_result in exclusions_array.values() {
            let exclusion_string = exclusion_result.unwrap();
            if exclusion_string.is_string() {
                exclusions.push(exclusion_string.as_string().unwrap());
            }
        }

        self.exclusions = Some(exclusions.join("|"));
    }

    /// Add cookierules to the ruleset from a cookierules array
    fn add_cookierules(&mut self, cookierules_array: &Array){
        let mut cookierules = vec![];
        for cookierule_result in cookierules_array.values() {
            let cookierule_obj = cookierule_result.unwrap();
            if cookierule_obj.is_object() {
                JS_STRINGS.with(|jss| {
                    let host_string = Reflect::get(&cookierule_obj, &jss.host).unwrap();
                    let name_string = Reflect::get(&cookierule_obj, &jss.name).unwrap();
                    if host_string.is_string() && name_string.is_string() {
                        cookierules.push(
                            CookieRule::new(
                                host_string.as_string().unwrap(),
                                name_string.as_string().unwrap()));
                    }
                });
            }
        }

        self.cookierules = Some(cookierules);
    }

    /// Return a bool indicating whether the given JS ruleset is equivalent
    fn is_equivalent_to(&self, ruleset_jsval: &JsValue) -> bool {
        let mut result = false;

        if ruleset_jsval.is_object() {
            JS_STRINGS.with(|jss| {
                let name_jsval = Reflect::get(&ruleset_jsval, &jss.name).unwrap();
                let note_jsval = Reflect::get(&ruleset_jsval, &jss.note).unwrap();
                let active_jsval = Reflect::get(&ruleset_jsval, &jss.active).unwrap();
                let rules_jsval = Reflect::get(&ruleset_jsval, &jss.rules).unwrap();

                let name_is_equiv = name_jsval.is_string() && name_jsval.as_string().unwrap() == self.name;
                let note_is_equiv = (note_jsval.is_null() && self.note == None) ||
                    (note_jsval.is_string() && self.note == Some(note_jsval.as_string().unwrap()));
                let active_is_equiv = Boolean::from(active_jsval) == self.active;

                if name_is_equiv && note_is_equiv && active_is_equiv && Array::is_array(&rules_jsval) {
                    let rules_array = Array::from(&rules_jsval);
                    if rules_array.length() == self.rules.len() as u32 {
                        let mut each_rule_equiv = true;
                        let mut counter = 0;
                        for rule_result in rules_array.values() {
                            let rule_jsval = rule_result.unwrap();
                            if rule_jsval.is_object() {
                                let to_jsval = Reflect::get(&rule_jsval, &jss.to).unwrap();
                                if let Some(to_string) = to_jsval.as_string() {
                                    match &self.rules[counter] {
                                        Rule::Trivial => {
                                            each_rule_equiv = to_string == "https:";
                                        },
                                        Rule::NonTrivial(_, this_to_val) => {
                                            each_rule_equiv = &to_string == this_to_val;
                                        }
                                    }
                                }
                                counter += 1;
                            } else {
                                each_rule_equiv = false;
                            }
                        }
                        result = each_rule_equiv;
                    }
                }
            });
        }
        result
    }

}


/// A newtype for rulesets, wrapping all the JS functionality
#[wasm_bindgen]
#[derive(Debug)]
pub struct JsRuleSets(RuleSets);

#[wasm_bindgen]
impl JsRuleSets {

    /// Returns a new JsRulesets struct
    pub fn new() -> JsRuleSets {
        JsRuleSets(RuleSets::new())
    }

    /// Returns the number of targets in the current JsRuleSets struct as a `usize`
    pub fn count_targets(&self) -> usize {
        self.0.count_targets()
    }

    /// Construct and add new rulesets given a JS array of values
    ///
    /// # Arguments
    ///
    /// * `array` - A JS Array object of rulesets
    /// * `enable_mixed_rulesets` - A JS Boolean indicating whether rulesets which trigger mixed
    /// content blocking should be enabled
    /// * `rule_active_states` - A JS object which lets us know whether rulesets have been disabled
    /// or enabled
    /// * `scope` - An optional JS string which indicates the scope of the current batch of
    /// rulesets being added (see the [ruleset update channels](https://github.com/EFForg/https-everywhere/blob/master/docs/en_US/ruleset-update-channels.md) documentation)
    pub fn add_all_from_js_array(&mut self, array: &Array, enable_mixed_rulesets: &Boolean, rule_active_states: &JsValue, scope: &JsValue) {

        let scope: Rc<Option<String>> = if scope.is_string() {
            Rc::new(Some(scope.as_string().unwrap()))
        } else {
            Rc::new(None)
        };

        let mut add_one_from_js = |jsval| {
            JS_STRINGS.with(|jss| {
                let mut ruleset_name: String;
                let mut default_state = true;
                let mut note = String::new();

                let default_off = Reflect::get(&jsval, &jss.default_off).unwrap();
                if default_off.is_string() {
                    if default_off != jss.user_rule {
                        default_state = false;
                    }
                    if let Some(default_off_string) = default_off.as_string() {
                        note += &(default_off_string + "\n");
                    }
                }

                let platform = Reflect::get(&jsval, &jss.platform).unwrap();
                if platform.is_string() {
                    if platform == jss.mixed_content {
                         if !enable_mixed_rulesets.value_of() {
                            default_state = false;
                        }
                    } else if !platform.is_undefined() {
                        default_state = false;
                    }
                    if let Some(platform_string) = platform.as_string() {
                        note.push_str("Platform(s): ");
                        note += &(platform_string + "\n");
                    }
                }

                let mut active = default_state;
                let name = Reflect::get(&jsval, &jss.name).unwrap();
                if name.is_string() {
                    ruleset_name = name.as_string().unwrap();
                    if rule_active_states.is_object() {
                        let active_state = Reflect::get(&rule_active_states, &JsValue::from_str(&ruleset_name)).unwrap();
                        match active_state.as_bool() {
                            Some(false) => { active = false; }
                            Some(true) => { active = true; }
                            _ => {}
                        }
                    }

                    let mut rs = RuleSet::new(ruleset_name, Rc::clone(&scope));
                    rs.default_state = default_state;
                    rs.note = match note.len() {
                        0 => None,
                        _ => Some(note.trim().to_string())
                    };
                    rs.active = active;

                    if let Ok(rules_jsval) = Reflect::get(&jsval, &jss.rule) {
                        if Array::is_array(&rules_jsval) {
                            rs.add_rules(&Array::from(&rules_jsval));
                        }
                    }

                    if let Ok(exclusion_jsval) = Reflect::get(&jsval, &jss.exclusion) {
                        if Array::is_array(&exclusion_jsval) {
                            rs.add_exclusions(&Array::from(&exclusion_jsval));
                        }
                    }

                    if let Ok(securecookie_jsval) = Reflect::get(&jsval, &jss.securecookie) {
                        if Array::is_array(&securecookie_jsval) {
                            rs.add_cookierules(&Array::from(&securecookie_jsval));
                        }
                    }

                    let rs_rc = Rc::new(rs);
                    let target_jsval = Reflect::get(&jsval, &jss.target).unwrap();
                    if Array::is_array(&target_jsval) {
                        for target_result in &Array::from(&target_jsval).values() {
                            let target = target_result.unwrap();
                            if target.is_string() {
                                let target = target.as_string().unwrap();
                                match (self.0).0.get_mut(&target) {
                                    Some(rs_vector) => {
                                        rs_vector.push(Rc::clone(&rs_rc));
                                    },
                                    None => {
                                        (self.0).0.insert(target, vec![Rc::clone(&rs_rc)]);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        };

        for val in array.values() {
            let jsval = val.unwrap();
            if jsval.is_object() {
                add_one_from_js(jsval);
            }
        }
    }

    #[cfg(debug_assertions)]
    /// Print the entire RuleSets struct
    pub fn print_targets (&self) {
        console_log!("{:?}", (self.0).0);
    }

    /// Remove a RuleSet from the RuleSets struct
    pub fn remove_ruleset (&mut self, ruleset_jsval: &JsValue) {
        if ruleset_jsval.is_object() {
            JS_STRINGS.with(|jss| {
                if let Ok(name) = Reflect::get(&ruleset_jsval, &jss.name) {
                    if name.is_string() {
                        let name = name.as_string().unwrap();
                        if (self.0).0.contains_key(&name) {
                            let ruleset_vec = (self.0).0.remove(&name).unwrap();
                            let mut new_ruleset_vec = vec![];

                            for ruleset in ruleset_vec {
                                if !ruleset.is_equivalent_to(ruleset_jsval) {
                                    new_ruleset_vec.push(Rc::clone(&ruleset));
                                }
                            }

                            if new_ruleset_vec.len() > 0 {
                                (self.0).0.insert(name, new_ruleset_vec);
                            }
                        }
                    }
                }
            });
        }
    }

    /// Return a JS set of rulesets that apply to the given host
    ///
    /// # Arguments
    ///
    /// * `host` - A JS string which indicates the host to search for potentially applicable rulesets
    pub fn potentially_applicable (&self, host: &JsValue) -> JsValue {
        if host.is_string() {
            let host = host.as_string().unwrap();
            self.0.potentially_applicable(&host).to_javascript()
        } else {
            Set::new(&Array::new()).into()
        }
    }
}
