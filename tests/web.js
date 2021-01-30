const trivial_rule = {from: "^http:", to: "https:"};
const trivial_rule_json = JSON.stringify(trivial_rule);
const nontrivial_rule = {from: "^http://example\\.com/", to: "https://example.com/"};
const nontrivial_rule_json = JSON.stringify(nontrivial_rule);
const rules = [trivial_rule, nontrivial_rule];
const cookierule_simple = {host: ".+", name: ".+"};
const cookierule_complicated = {"host":"^maps\\.gstatic\\.com$","name":".+"};
const cookierule_json = JSON.stringify(cookierule_simple);
const cookierules = [cookierule_simple, cookierule_complicated];
const exclusions = ["^http://maps\\.gstatic\\.com/something$", "^http://maps\\.example\\.com/test"];
const ruleset = {"name":"Example Ruleset","active":false,"default_state":true,"scope":"^http://www\\.example\\.com/","note":"Some note","rules":[{"from":"^http:","to":"https:"},{"from":"^http://example\\.com/","to":"https://example.com/"}],"exclusions":"^http://maps\\.gstatic\\.com/something$|^http://maps\\.example\\.com/test","cookierules":[{"host":".+","name":".+"},{"host":"^maps\\.gstatic\\.com$","name":".+"}]};
const ruleset_json = JSON.stringify(ruleset);
const roughly_equivalent_ruleset = {"name":"Example Ruleset","active":false,"note":"Some note","rules":[{"from":"^http:","to":"https:"},{"from":"^http://example\\.com/","to":"https://example.com/"}]};
const nonequivalent_ruleset_1 = {"name":"Nonequivalent Example Ruleset","active":false,"note":"Some note","rules":[{"from":"^http:","to":"https:"},{"from":"^http://example\\.com/","to":"https://example.com/"}]};
const nonequivalent_ruleset_2 = {"name":"Example Ruleset","active":true,"note":"Some note","rules":[{"from":"^http:","to":"https:"},{"from":"^http://example\\.com/","to":"https://example.com/"}]};
const nonequivalent_ruleset_3 = {"name":"Example Ruleset","active":true,"note":"Some note","rules":[{"from":"^http:","to":"https:"},{"from":"^http://example\\.com/test","to":"https://example.com/"}]};
const rulesets = [{"name":"1fichier","default_off":"failed ruleset test","target":["1fichier.com","*.1fichier.com","alterupload.com","cjoint.net","desfichiers.com","megadl.fr","mesfichiers.org","piecejointe.net","pjointe.com","tenvoi.com","dl4free.com"],"rule":[{"from":"^http:","to":"https:"}]},{"name":"Freerangekitten.com","target":["freerangekitten.com","www.freerangekitten.com"],"rule":[{"from":"^http:","to":"https:"}]},{"name":"Google APIs","target":["gmodules.com","www.gmodules.com","ajax.googleapis.com","chart.googleapis.com","ct.googleapis.com","fonts.googleapis.com","imasdk.googleapis.com","maps.googleapis.com","www.googleapis.com","commondatastorage.googleapis.com","*.commondatastorage.googleapis.com","*.storage.googleapis.com","storage.googleapis.com","gstatic.com","*.gstatic.com"],"exclusion":["^http://(www\\.)?gmodules\\.com/$|^http://ajax\\.googleapis\\.com/$|^http://chart\\.googleapis\\.com/$|^http://www\\.gstatic\\.com/generate_204|^http://www\\.gstatic\\.com/ddos-viz/attacks\\.json|^http://codeskulptor-user\\d+\\.commondatastorage\\.googleapis\\.com/"],"securecookie":[{"host":"^maps\\.gstatic\\.com$","name":".+"}],"rule":[{"from":"^http://(?:www\\.)?gmodules\\.com/ig/images/","to":"https://www.google.com/ig/images/"},{"from":"^http://(ajax|chart|ct|fonts|imasdk|maps|www)\\.googleapis\\.com/","to":"https://$1.googleapis.com/"},{"from":"^http://([\\w-]+\\.)?(commondata)?storage\\.googleapis\\.com/","to":"https://$1$2storage.googleapis.com/"},{"from":"^http://(www\\.)?gstatic\\.com/","to":"https://www.gstatic.com/"},{"from":"^http://(csi|encrypted-tbn\\d|fonts|g0|maps|[\\w-]+\\.metric|ssl|t\\d)\\.gstatic\\.com/","to":"https://$1.gstatic.com/"}]}];
const enable_mixed_rulesets = true;
const ruleset_active_states = {"1fichier": true, "Google APIs": false};
const scope = "^http://example\\.com/";
const get_simple_rules_ending_with_result_json_1 = JSON.stringify([{"host":"*.1fichier.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"1fichier.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"alterupload.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"desfichiers.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"dl4free.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"freerangekitten.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"pjointe.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"tenvoi.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"},{"host":"www.freerangekitten.com","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"}]);
const get_simple_rules_ending_with_result_json_2 = JSON.stringify([{"host":"mesfichiers.org","from_regex":"/^http:/","to":"https:","scope_regex":"/^http:\\/\\/example\\.com\\//"}]);
const potentially_applicable_result_json_1 = JSON.stringify([{
  name: 'Google APIs',
  active: false,
  default_state: true,
  scope: '^http://example\\.com/',
  rules:[
    { from: '^http://(?:www\\.)?gmodules\\.com/ig/images/', to: 'https://www.google.com/ig/images/' },
    { from: '^http://(ajax|chart|ct|fonts|imasdk|maps|www)\\.googleapis\\.com/', to: 'https://$1.googleapis.com/' },
    { from: '^http://([\\w-]+\\.)?(commondata)?storage\\.googleapis\\.com/', to: 'https://$1$2storage.googleapis.com/' },
    { from: '^http://(www\\.)?gstatic\\.com/', to: 'https://www.gstatic.com/' },
    { from: '^http://(csi|encrypted-tbn\\d|fonts|g0|maps|[\\w-]+\\.metric|ssl|t\\d)\\.gstatic\\.com/', to: 'https://$1.gstatic.com/' }
  ],
  exclusions: '^http://(www\\.)?gmodules\\.com/$|^http://ajax\\.googleapis\\.com/$|^http://chart\\.googleapis\\.com/$|^http://www\\.gstatic\\.com/generate_204|^http://www\\.gstatic\\.com/ddos-viz/attacks\\.json|^http://codeskulptor-user\\d+\\.commondatastorage\\.googleapis\\.com/',
  cookierules: [ { host: '^maps\\.gstatic\\.com$', name: '.+' } ]
}]);
const potentially_applicable_result_json_2 = JSON.stringify([{
  name: 'example.com',
  active: true,
  default_state: true,
  scope: '^http://example\\.com/',
  note: 'user rule',
  rules: [ { from: '^http://example\\.com/', to: 'https:' } ]
}]);
const added_user_rule = [{"name":"example.com","target":["example.com"],"rule":[{"to":"https:","from":"^http://example\\.com/"}],"default_off":"user rule"}];
const removed_user_rule = {"name":"example.com","rules":[{"from_c":{},"to":"https:"}],"exclusions":null,"cookierules":null,"active":true,"default_state":true,"scope":{},"note":"user rule"};
const bloom_args = [new Uint8Array([53, 89, 153, 27]), 32, 8, [["7042164324123011823", "4838412312024205537"], ["13508439373888375696", "10975652807371557587"]]];

module.exports = {
  trivial_rule_json: () => {
    return trivial_rule_json;
  },
  nontrivial_rule_json: () => {
    return nontrivial_rule_json;
  },
  rules: () => {
    return rules;
  },
  cookierule_json: () => {
    return cookierule_json;
  },
  cookierules: () => {
    return cookierules;
  },
  exclusions: () => {
    return exclusions;
  },
  ruleset_json: () => {
    return ruleset_json;
  },
  roughly_equivalent_ruleset: () => {
    return roughly_equivalent_ruleset;
  },
  nonequivalent_ruleset_1: () => {
    return nonequivalent_ruleset_1;
  },
  nonequivalent_ruleset_2: () => {
    return nonequivalent_ruleset_2;
  },
  nonequivalent_ruleset_3: () => {
    return nonequivalent_ruleset_3;
  },
  rulesets: () => {
    return rulesets;
  },
  enable_mixed_rulesets: () => {
    return enable_mixed_rulesets;
  },
  ruleset_active_states: () => {
    return ruleset_active_states;
  },
  scope: () => {
    return scope;
  },
  get_simple_rules_ending_with_result_json_1: () => {
    return get_simple_rules_ending_with_result_json_1;
  },
  get_simple_rules_ending_with_result_json_2: () => {
    return get_simple_rules_ending_with_result_json_2;
  },
  potentially_applicable_result_json_1: () => {
    return potentially_applicable_result_json_1;
  },
  potentially_applicable_result_json_2: () => {
    return potentially_applicable_result_json_2;
  },
  added_user_rule: () => {
    return added_user_rule;
  },
  removed_user_rule: () => {
    return removed_user_rule;
  },
  bloom_args: () => {
    return bloom_args;
  },
};
