use bevy::prelude::*;
use bevy::utils::hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
pub const X_EXTENT: f32 = 600.;

#[derive(Event)]
pub struct FactUpdated {
    pub fact: Fact,
}

#[derive(Event)]
pub struct RuleUpdated {
    pub rule: String,
}

// Fact enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Fact {
    Int(String, i32),
    String(String, String),
    Bool(String, bool),
    StringList(String, StringHashSet),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct StringHashSet(HashSet<String>);

impl StringHashSet {
    fn new() -> Self {
        StringHashSet(HashSet::new())
    }

    fn insert(&mut self, value: String) -> bool {
        self.0.insert(value)
    }

    fn remove(&mut self, value: &String) -> bool {
        self.0.remove(value)
    }
}

impl Hash for StringHashSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted: Vec<&String> = self.0.iter().collect();
        sorted.sort();
        for item in sorted {
            item.hash(state);
        }
    }
}

#[derive(Resource, Deserialize, Serialize)]
pub struct CoolFactStore {
    pub facts: HashMap<String, Fact>,
    pub updated_facts: HashSet<Fact>,
}

impl CoolFactStore {
    pub fn new() -> Self {
        CoolFactStore {
            facts: HashMap::new(),
            updated_facts: HashSet::new(),
        }
    }

    pub fn store_int(&mut self, key: String, value: i32) {
        if let Some(fact) = self.facts.get_mut(&key) {
            if let Fact::Int(_, current_value) = fact {
                if current_value != &value {
                    *fact = Fact::Int(key.clone(), value);
                    self.updated_facts.insert(fact.clone());
                }
            } else {
                panic!("Fact with key {} is not an integer", key)
            }
        } else {
            self.facts
                .insert(key.clone(), Fact::Int(key.clone(), value));
            self.updated_facts.insert(Fact::Int(key.clone(), value));
        }
    }

    pub fn add_to_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    fn subtract_from_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    pub fn store_string(&mut self, key: String, value: String) {
        if let Some(fact) = self.facts.get_mut(&key) {
            if let Fact::String(_, current_value) = fact {
                if current_value != &value {
                    *fact = Fact::String(key.clone(), value.clone());
                    self.updated_facts.insert(fact.clone());
                }
            } else {
                panic!("Fact with key {} is not a string", key)
            }
        } else {
            self.facts
                .insert(key.clone(), Fact::String(key.clone(), value.clone()));
            self.updated_facts
                .insert(Fact::String(key.clone(), value.clone()));
        }
    }

    pub fn store_bool(&mut self, key: String, value: bool) {
        if let Some(fact) = self.facts.get_mut(&key) {
            if let Fact::Bool(_, current_value) = fact {
                if current_value != &value {
                    *fact = Fact::Bool(key.clone(), value);
                    self.updated_facts.insert(fact.clone());
                }
            } else {
                panic!("Fact with key {} is not a boolean", key)
            }
        } else {
            self.facts
                .insert(key.clone(), Fact::Bool(key.clone(), value.clone()));
            self.updated_facts
                .insert(Fact::Bool(key.clone(), value.clone()));
        }
    }

    pub fn add_to_list(&mut self, key: String, value: String) {
        if let Some(list_fact) = self.facts.get_mut(&key) {
            if let Fact::StringList(_, list) = list_fact {
                if list.insert(value) {
                    self.updated_facts.insert(list_fact.clone());
                }
            }
        } else {
            let mut new_list = StringHashSet::new();
            new_list.insert(value);
            self.facts
                .insert(key.clone(), Fact::StringList(key.clone(), new_list.clone()));
            self.updated_facts
                .insert(Fact::StringList(key.clone(), new_list.clone()));
        }
    }

    pub fn remove_from_list(&mut self, key: String, value: String) {
        if let Some(list_fact) = self.facts.get_mut(&key) {
            if let Fact::StringList(_, list) = list_fact {
                if list.remove(&value) {
                    self.updated_facts.insert(list_fact.clone());
                }
            }
        }
    }

    pub fn get_int(&self, key: &str) -> Option<&i32> {
        return if let Some(Fact::Int(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    pub fn get_string(&self, key: &str) -> Option<&String> {
        return if let Some(Fact::String(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    pub fn get_bool(&self, key: &str) -> Option<&bool> {
        return if let Some(Fact::Bool(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    pub fn get_list(&self, key: &str) -> Option<&StringHashSet> {
        return if let Some(Fact::StringList(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }
}

// Condition enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Condition {
    IntEquals {
        fact_name: String,
        expected_value: i32,
    },
    IntMoreThan {
        fact_name: String,
        expected_value: i32,
    },
    IntLessThan {
        fact_name: String,
        expected_value: i32,
    },
    StringEquals {
        fact_name: String,
        expected_value: String,
    },
    BoolEquals {
        fact_name: String,
        expected_value: bool,
    },
    ListContains {
        fact_name: String,
        expected_value: String,
    },
}

impl Condition {
    pub fn evaluate(&self, facts: &HashMap<String, Fact>) -> bool {
        match self {
            Condition::IntEquals {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value == *expected_value;
                }
            }
            Condition::StringEquals {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::String(_, value)) = facts.get(fact_name) {
                    return value == expected_value;
                }
            }
            Condition::BoolEquals {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::Bool(_, value)) = facts.get(fact_name) {
                    return *value == *expected_value;
                }
            }
            Condition::IntMoreThan {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value > *expected_value;
                }
            }
            Condition::IntLessThan {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value < *expected_value;
                }
            }
            Condition::ListContains {
                fact_name,
                expected_value,
            } => {
                if let Some(Fact::StringList(_, value)) = facts.get(fact_name) {
                    return value.0.contains(expected_value);
                }
            }
        }
        false
    }
}

// Rule struct
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Rule {
    pub name: String,
    pub conditions: Vec<Condition>,
}

impl Rule {
    pub fn new(name: String, conditions: Vec<Condition>) -> Self {
        Rule { name, conditions }
    }

    pub fn evaluate(&self, facts: &HashMap<String, Fact>) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.evaluate(facts))
    }
}

// StoryBeat struct
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct StoryBeat {
    pub name: String,
    pub rules: Vec<Rule>,
    pub finished: bool,
}

impl StoryBeat {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        StoryBeat {
            name,
            rules,
            finished: false,
        }
    }

    pub fn evaluate(&mut self, facts: &HashMap<String, Fact>) {
        self.finished = self.rules.iter().all(|rule| rule.evaluate(facts));
    }
}

// Story struct
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Story {
    pub name: String,
    pub beats: Vec<StoryBeat>,
    pub active_beat_index: usize,
}

impl Story {
    pub fn new(name: String, beats: Vec<StoryBeat>) -> Self {
        Story {
            name,
            beats,
            active_beat_index: 0,
        }
    }

    pub fn evaluate_active_beat(&mut self, facts: &HashMap<String, Fact>) {
        if self.active_beat_index < self.beats.len() {
            let active_beat = &mut self.beats[self.active_beat_index];
            active_beat.evaluate(facts);
            if active_beat.finished {
                self.active_beat_index += 1;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.active_beat_index >= self.beats.len()
    }
}

// StoryEngine struct
#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct StoryEngine {
    pub stories: Vec<Story>,
}

impl StoryEngine {
    pub fn new() -> Self {
        StoryEngine {
            stories: Vec::new(),
        }
    }

    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
    }

    pub fn evaluate_stories(&mut self, facts: &HashMap<String, Fact>) {
        for story in &mut self.stories {
            story.evaluate_active_beat(facts);
        }
    }

    pub fn all_stories_finished(&self) -> bool {
        self.stories.iter().all(|story| story.is_finished())
    }
}

// Builder Pattern
pub struct FactBuilder {
    pub key: String,
}

impl FactBuilder {
    pub fn new(key: String) -> Self {
        FactBuilder { key }
    }

    pub fn int(self, value: i32) -> Fact {
        Fact::Int(self.key, value)
    }

    pub fn string(self, value: String) -> Fact {
        Fact::String(self.key, value)
    }

    pub fn bool(self, value: bool) -> Fact {
        Fact::Bool(self.key, value)
    }

    pub fn string_list(self, values: Vec<String>) -> Fact {
        let mut set = StringHashSet::new();
        for value in values {
            set.insert(value);
        }
        Fact::StringList(self.key, set)
    }
}

pub struct ConditionBuilder {
    pub conditions: Vec<Condition>,
}

impl ConditionBuilder {
    pub fn new() -> Self {
        ConditionBuilder {
            conditions: Vec::new(),
        }
    }

    pub fn int_equals(mut self, fact_name: String, expected_value: i32) -> Self {
        self.conditions.push(Condition::IntEquals {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn int_more_than(mut self, fact_name: String, expected_value: i32) -> Self {
        self.conditions.push(Condition::IntMoreThan {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn int_less_than(mut self, fact_name: String, expected_value: i32) -> Self {
        self.conditions.push(Condition::IntLessThan {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn string_equals(mut self, fact_name: String, expected_value: String) -> Self {
        self.conditions.push(Condition::StringEquals {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn bool_equals(mut self, fact_name: String, expected_value: bool) -> Self {
        self.conditions.push(Condition::BoolEquals {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn list_contains(mut self, fact_name: String, expected_value: String) -> Self {
        self.conditions.push(Condition::ListContains {
            fact_name,
            expected_value,
        });
        self
    }

    pub fn build(self) -> Vec<Condition> {
        self.conditions
    }
}

pub struct RuleBuilder {
    name: String,
    conditions: Vec<Condition>,
}

impl RuleBuilder {
    pub fn new(name: String) -> Self {
        RuleBuilder {
            name,
            conditions: Vec::new(),
        }
    }

    pub fn conditions(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions = conditions;
        self
    }

    pub fn build(self) -> Rule {
        Rule {
            name: self.name,
            conditions: self.conditions,
        }
    }
}

pub struct StoryBeatBuilder {
    name: String,
    rules: Vec<Rule>,
}

impl StoryBeatBuilder {
    pub fn new(name: String) -> Self {
        StoryBeatBuilder {
            name,
            rules: Vec::new(),
        }
    }

    pub fn rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn build(self) -> StoryBeat {
        StoryBeat {
            name: self.name,
            rules: self.rules,
            finished: false,
        }
    }
}

pub struct StoryBuilder {
    name: String,
    beats: Vec<StoryBeat>,
}

impl StoryBuilder {
    pub fn new(name: String) -> Self {
        StoryBuilder {
            name,
            beats: Vec::new(),
        }
    }

    pub fn beats(mut self, beats: Vec<StoryBeat>) -> Self {
        self.beats = beats;
        self
    }

    pub fn build(self) -> Story {
        Story {
            name: self.name,
            beats: self.beats,
            active_beat_index: 0,
        }
    }
}

pub struct StoryEngineBuilder {
    stories: Vec<Story>,
}

impl StoryEngineBuilder {
    pub fn new() -> Self {
        StoryEngineBuilder {
            stories: Vec::new(),
        }
    }

    pub fn stories(mut self, stories: Vec<Story>) -> Self {
        self.stories = stories;
        self
    }

    pub fn build(self) -> StoryEngine {
        StoryEngine {
            stories: self.stories,
        }
    }
}

#[derive(Resource, Deserialize, Serialize)]
pub struct RuleEngine {
    pub rules: HashMap<String, Rule>,
    pub rule_states: HashMap<String, bool>,
}

impl RuleEngine {
    // Constructor for RuleEngine
    pub fn new() -> Self {
        RuleEngine {
            rules: HashMap::new(),
            rule_states: HashMap::new(),
        }
    }

    // Add a new rule to the rule engine
    pub fn add_rule(&mut self, rule: Rule) {
        self.rule_states.insert(rule.name.clone(), false);
        self.rules.insert(rule.name.clone(), rule);
    }

    // Evaluate all rules based on the provided facts
    pub fn evaluate_rules(&mut self, facts: &HashMap<String, Fact>) -> HashSet<String> {
        let mut updated_rule_states = HashSet::new();
        self.rules.iter().for_each(|(name, rule)| {
            let previous_state = self.rule_states.get(name).unwrap();
            if previous_state != &rule.evaluate(facts) {
                self.rule_states.insert(name.clone(), !previous_state);
                updated_rule_states.insert(name.clone());
            }
        });
        updated_rule_states
    }
}
