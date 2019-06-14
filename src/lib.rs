use wasm_bindgen::prelude::*;
use js_sys::{Array, Reflect, Boolean, Set, Object};
use std::collections::BTreeMap;
use std::rc::Rc;

mod debugging;

const ERR: &str = "could not convert property to JS";

#[derive(Debug)]
pub struct StaticJsStrings {
    pub active: JsValue,
    pub cookierules: JsValue,
    pub default_off: JsValue,
    pub default_state: JsValue,
    pub exclusion: JsValue,
    pub exclusions: JsValue,
    pub from: JsValue,
    pub host: JsValue,
    pub mixed_content: JsValue,
    pub name: JsValue,
    pub note: JsValue,
    pub platform: JsValue,
    pub rule: JsValue,
    pub rules: JsValue,
    pub scope: JsValue,
    pub securecookie: JsValue,
    pub state: JsValue,
    pub target: JsValue,
    pub to: JsValue,
    pub user_rule: JsValue,
}

thread_local! {
    pub static JS_STRINGS: StaticJsStrings = StaticJsStrings {
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

/// A Rule is used to rewrite URLs from some regular expression to some string
#[derive(Debug)]
pub enum Rule {
    Trivial,
    NonTrivial(String, String)
}

impl Rule {

    /// Returns a rule with the from regex and replacement string specified
    ///
    /// # Arguments
    ///
    /// * `from_regex` - A string that will be compiled to regex indicating the URL to replace
    /// * `to` - A string indicating the replacement value
    pub fn new(from_regex: String, to: String) -> Rule {
        if &from_regex == "^http:" && &to == "https:" {
            Rule::Trivial
        } else {
            Rule::NonTrivial(from_regex, to)
        }
    }

    /// Convert a rule to a JS object
    pub fn to_js_object(&self) -> Object {
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
        object
    }
}


/// A CookieRule is used to secure cookies which conform to some name and host constraints
#[derive(Debug)]
pub struct CookieRule {
    host_regex: String, // RegExp
    name_regex: String // RegExp
}

impl CookieRule {

    /// Returns a cookierule with the host and scope regex specified
    ///
    /// # Arguments
    ///
    /// * `host_regex` - A string that will be compiled to regex indicating the host of the cookie
    /// * `name_regex` - A string that will be compiled to regex indicating the name of the cookie
    pub fn new(host_regex: String, name_regex: String) -> CookieRule {
        CookieRule {
            host_regex,
            name_regex
        }
    }

    /// Convert a ruleset to a JS object
    pub fn to_js_object(&self) -> Object {
        let object = Object::new();
        JS_STRINGS.with(|jss| {
            Reflect::set(&object, &jss.host, &JsValue::from(&self.host_regex)).expect(ERR);
            Reflect::set(&object, &jss.name, &JsValue::from(&self.name_regex)).expect(ERR);
        });
        object
    }
}

/// A RuleSet is a grouping of rules which act on some target
#[derive(Debug)]
pub struct RuleSet {
    pub name: String,
    rules: Vec<Rule>,
    exclusions: Option<String>, // RegExp
    cookierules: Option<Vec<CookieRule>>,
    pub active: bool,
    pub default_state: bool,
    pub scope: Rc<Option<String>>, // RegExp
    pub note: Option<String>
}

impl RuleSet {

    /// Returns a ruleset with the name and scope specified
    ///
    /// # Arguments
    ///
    /// * `name` - A string that holds the name of the ruleset
    /// * `scope` - An optional string slice specifying the scope of the ruleset
    pub fn new(name: String, scope: Rc<Option<String>>) -> RuleSet {
        RuleSet {
            name,
            rules: vec![],
            exclusions: None,
            cookierules: None,
            active: true,
            default_state: true,
            scope,
            note: None
        }
    }

    /// Add rules, specified in JS array
    pub fn add_rules(&mut self, rules_array: &Array){
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
    pub fn add_exclusions(&mut self, exclusions_array: &Array){
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
    pub fn add_cookierules(&mut self, cookierules_array: &Array){
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
    pub fn is_equivalent_to(&self, ruleset_jsval: &JsValue) -> bool {
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

    /// Convert a ruleset to a JS object
    pub fn to_js_object(&self) -> Object {
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
                rules.push(&rule.to_js_object());
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
                        cookierules_array.push(&cookierule.to_js_object());
                    }
                    Reflect::set(&object, &jss.cookierules, &cookierules_array).expect(ERR);
                },
                None => {}
            }
        });
        object
    }
}

/// RuleSets consists of a tuple hashmap of rulesets, keyed by some target FQDN
#[wasm_bindgen]
#[derive(Debug)]
pub struct RuleSets(BTreeMap<String, Vec<Rc<RuleSet>>>);

#[wasm_bindgen]
impl RuleSets {

    /// Returns a new rulesets struct
    pub fn new() -> RuleSets {
        RuleSets(BTreeMap::new())
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
                                match self.0.get_mut(&target) {
                                    Some(rs_vector) => {
                                        rs_vector.push(Rc::clone(&rs_rc));
                                    },
                                    None => {
                                        self.0.insert(target, vec![Rc::clone(&rs_rc)]);
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

    /// Returns the number of targets in the current RuleSets struct as a `usize`
    pub fn count_targets (&self) -> usize {
        self.0.len()
    }

    #[cfg(debug_assertions)]
    /// Print the entire RuleSets struct
    pub fn print_targets (&self) {
        console_log!("{:?}", self.0);
    }

    /// Remove a RuleSet from the RuleSets struct
    pub fn remove_ruleset (&mut self, ruleset_jsval: &JsValue) {
        if ruleset_jsval.is_object() {
            JS_STRINGS.with(|jss| {
                if let Ok(name) = Reflect::get(&ruleset_jsval, &jss.name) {
                    if name.is_string() {
                        let name = name.as_string().unwrap();
                        if self.0.contains_key(&name) {
                            let ruleset_vec = self.0.remove(&name).unwrap();
                            let mut new_ruleset_vec = vec![];

                            for ruleset in ruleset_vec {
                                if !ruleset.is_equivalent_to(ruleset_jsval) {
                                    new_ruleset_vec.push(Rc::clone(&ruleset));
                                }
                            }

                            if new_ruleset_vec.len() > 0 {
                                self.0.insert(name, new_ruleset_vec);
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
    pub fn potentially_applicable (&self, host: &JsValue) -> Set {
        let results = Set::new(&Array::new());

        let try_add = |host: &String| {
            if self.0.contains_key(host) {
                if let Some(rulesets) = self.0.get(host) {
                    for ruleset in rulesets {
                        results.add(&ruleset.to_js_object());
                    }
                }
            }
        };

        if host.is_string() {
            let host = host.as_string().unwrap();

            // Try adding the full host
            try_add(&host);

            // Ensure host is well-formed (RFC 1035)
            if host.len() <= 0 || host.len() > 255 || host.find("..").is_some() {
                return results;
            }

            // Replace www.example.com with www.example.*
            // eat away from the right for once and only once
            let mut segmented: Vec<&str> = host.split('.').collect();
            let last_index = segmented.len() - 1;
            let tld = segmented[last_index];

            segmented[last_index] = "*";
            let tmp_host = segmented.join(".");
            try_add(&tmp_host);
            segmented[last_index] = tld;

            // now eat away from the left, with *, so that for x.y.z.google.com we
            // check *.y.z.google.com, *.z.google.com and *.google.com
            for index in 0..(segmented.len() - 1) {
                let mut segmented_tmp = segmented.clone();
                segmented_tmp[index] = "*";
                let tmp_host = segmented_tmp.join(".");
                try_add(&tmp_host);
            }
        }

        results
    }
}
