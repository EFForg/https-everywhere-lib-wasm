mod rulesets;
pub use rulesets::RuleSets;
mod bloom;
pub use bloom::Bloom;

// the following are used for tests
pub use rulesets::{Rule,CookieRule,RuleSet,JsRuleSet,ToJavaScript};

//// Turn on console logging in web browser (but messes up tests)
//#[cfg(debug_assertions)]
//mod log;
