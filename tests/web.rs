use std::rc::Rc;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use js_sys::{Array,Boolean};
use https_everywhere_lib_wasm::{Rule,CookieRule,RuleSet,JsRuleSet,JsRuleSets,ToJavaScript};

#[macro_use]
extern crate matches;

#[wasm_bindgen(module = "/tests/web.js")]
extern "C" {
    fn rules() -> Array;
    fn trivial_rule_json() -> JsValue;
    fn nontrivial_rule_json() -> JsValue;
    fn cookierules() -> Array;
    fn cookierule_json() -> JsValue;
    fn exclusions() -> Array;
    fn ruleset_json() -> JsValue;
    fn roughly_equivalent_ruleset() -> JsValue;
    fn nonequivalent_ruleset_1() -> JsValue;
    fn nonequivalent_ruleset_2() -> JsValue;
    fn nonequivalent_ruleset_3() -> JsValue;
    fn rulesets() -> Array;
    fn enable_mixed_rulesets() -> Boolean;
    fn ruleset_active_states() -> JsValue;
    fn scope() -> JsValue;
    fn potentially_applicable_result_json_1() -> JsValue;
    fn potentially_applicable_result_json_2() -> JsValue;
    fn added_user_rule() -> Array;
    fn removed_user_rule() -> JsValue;
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = JSON)]
    pub fn stringify(s: &JsValue) -> JsValue;
}


// Rule tests
#[wasm_bindgen_test]
fn create_trivial_rule() {
    let trivial_rule = Rule::new(String::from("^http:"), String::from("https:"));
    assert_matches!(trivial_rule, Rule::Trivial);
}

#[wasm_bindgen_test]
fn create_nontrivial_rule() {
    let nontrivial_rule = Rule::new(String::from("^http://example\\.com/"), String::from("https://example.com/"));
    assert_matches!(nontrivial_rule, Rule::NonTrivial(_, _));
}

#[wasm_bindgen_test]
fn convert_trivial_rule_to_javascript() {
    let trivial_rule = Rule::new(String::from("^http:"), String::from("https:"));
    let converted_js_object = stringify(&trivial_rule.to_javascript());
    assert_eq!(converted_js_object, trivial_rule_json());
}

#[wasm_bindgen_test]
fn convert_nontrivial_rule_to_javascript() {
    let nontrivial_rule = Rule::new(String::from("^http://example\\.com/"), String::from("https://example.com/"));
    let converted_js_object = stringify(&nontrivial_rule.to_javascript());
    assert_eq!(converted_js_object, nontrivial_rule_json());
}


// CookieRule tests
#[wasm_bindgen_test]
fn convert_cookierule_to_javascript() {
    let cookierule = CookieRule::new(String::from(".+"), String::from(".+"));
    let converted_js_object = stringify(&cookierule.to_javascript());
    assert_eq!(converted_js_object, cookierule_json());
}


// RuleSet tests
fn mock_ruleset() -> RuleSet {
    let scope = Rc::new(Some(String::from("^http://www\\.example\\.com/")));
    let mut ruleset = RuleSet::new(String::from("Example Ruleset"), scope);
    ruleset.add_cookierules(&cookierules());
    ruleset.add_rules(&rules());
    ruleset.add_exclusions(&exclusions());
    ruleset.note = Some(String::from("Some note"));
    ruleset.active = false;
    ruleset
}

#[wasm_bindgen_test]
fn rough_equivalence() {
    assert_eq!(mock_ruleset().is_equivalent_to(&roughly_equivalent_ruleset()), true);
}

#[wasm_bindgen_test]
fn nonequivalence() {
    let mock_ruleset = mock_ruleset();
    assert_eq!(mock_ruleset.is_equivalent_to(&nonequivalent_ruleset_1()), false);
    assert_eq!(mock_ruleset.is_equivalent_to(&nonequivalent_ruleset_2()), false);
    assert_eq!(mock_ruleset.is_equivalent_to(&nonequivalent_ruleset_3()), false);
}

#[wasm_bindgen_test]
fn convert_ruleset_to_javascript() {
    let converted_js_object = stringify(&mock_ruleset().to_javascript());
    assert_eq!(converted_js_object, ruleset_json());
}


// JsRuleSets tests
fn mock_rulesets() -> JsRuleSets {
    let mut mock_rulesets = JsRuleSets::new();
    mock_rulesets.add_all_from_js_array(&rulesets(), &enable_mixed_rulesets(), &ruleset_active_states(), &scope());
    mock_rulesets
}

#[wasm_bindgen_test]
fn count_rulesets_targets() {
    assert_eq!(mock_rulesets().count_targets(), 28);
}

#[wasm_bindgen_test]
fn rulesets_potentially_applicable() {
    let potentially_applicable_result = mock_rulesets().potentially_applicable(&JsValue::from("gstatic.com"));
    assert_eq!(stringify(&Array::from(&potentially_applicable_result)), potentially_applicable_result_json_1());
}

#[wasm_bindgen_test]
fn add_rulesets() {
    let mut rulesets = mock_rulesets();
    rulesets.add_all_from_js_array(&added_user_rule(), &enable_mixed_rulesets(), &ruleset_active_states(), &scope());
    assert_eq!(rulesets.count_targets(), 29);

    let potentially_applicable_result = rulesets.potentially_applicable(&JsValue::from("example.com"));
    assert_eq!(stringify(&Array::from(&potentially_applicable_result)), potentially_applicable_result_json_2());
}

#[wasm_bindgen_test]
fn rulesets_potentially_applicable_after_remove() {
    let mut rulesets = mock_rulesets();
    rulesets.add_all_from_js_array(&added_user_rule(), &enable_mixed_rulesets(), &ruleset_active_states(), &scope());
    rulesets.remove_ruleset(&removed_user_rule());
    assert_eq!(rulesets.count_targets(), 28);

    let potentially_applicable_result = rulesets.potentially_applicable(&JsValue::from("example.com"));
    assert_eq!(stringify(&Array::from(&potentially_applicable_result)), JsValue::from("[]"));
}
