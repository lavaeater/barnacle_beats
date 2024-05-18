use std::hash::{Hash, Hasher};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
pub(crate) const X_EXTENT: f32 = 600.;

#[derive(Event)]
pub struct FactUpdated {
    pub(crate) fact: Fact,
}

#[derive(Event)]
pub struct RuleUpdated {
    pub(crate) rule: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Fact {
    Int(String, i32),
    String(String, String),
    Bool(String, bool),
    StringList(String, StringHashSet),
}

#[derive(Resource, Deserialize, Serialize)]
pub struct CoolFactStore {
    facts: HashMap<String, Fact>,
    updated_facts: HashSet<Fact>,
}

impl CoolFactStore {
    // Create a new instance of FactStore
    fn new() -> Self {
        CoolFactStore {
            facts: HashMap::new(),
            updated_facts: HashSet::new(),
        }
    }

    // Store an integer fact
    pub(crate) fn store_int(&mut self, key: String, value: i32) {
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
            self.facts.insert(key.clone(), Fact::Int(key.clone(), value));
            self.updated_facts.insert(Fact::Int(key.clone(), value));
        }
    }

    pub(crate) fn add_to_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    fn subtract_from_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    // Store a string fact
    pub(crate) fn store_string(&mut self, key: String, value: String) {
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
            self.facts.insert(key.clone(), Fact::String(key.clone(), value.clone()));
            self.updated_facts.insert(Fact::String(key.clone(), value.clone()));
        }
    }

    // Store a boolean fact
    pub(crate) fn store_bool(&mut self, key: String, value: bool) {
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
            self.facts.insert(key.clone(), Fact::Bool(key.clone(), value.clone()));
            self.updated_facts.insert(Fact::Bool(key.clone(), value.clone()));
        }
    }

    // Store a list of strings fact
    fn add_to_list(&mut self, key: String, value: String) {
        if let Some(list_fact) = self.facts.get_mut(&key) {
            if let Fact::StringList(_, list) = list_fact {
                if list.insert(value) {
                    self.updated_facts.insert(list_fact.clone());
                }
            }
        } else {
            let mut new_list = StringHashSet::new();
            new_list.insert(value);
            self.facts.insert(key.clone(), Fact::StringList(key.clone(), new_list.clone()));
            self.updated_facts.insert(Fact::StringList(key.clone(), new_list.clone()));
        }
    }

    fn remove_from_list(&mut self, key: String, value: String) {
        if let Some(list_fact) = self.facts.get_mut(&key) {
            if let Fact::StringList(_, list) = list_fact {
                if list.remove(&value) {
                    self.updated_facts.insert(list_fact.clone());
                }
            }
        }
    }

    // Retrieve an integer fact
    pub(crate) fn get_int(&self, key: &str) -> Option<&i32> {
        return if let Some(Fact::Int(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    // Retrieve a string fact
    fn get_string(&self, key: &str) -> Option<&String> {
        return if let Some(Fact::String(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    // Retrieve a boolean fact
    fn get_bool(&self, key: &str) -> Option<&bool> {
        return if let Some(Fact::Bool(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }

    // Retrieve a list of strings fact
    fn get_list(&self, key: &str) -> Option<&StringHashSet> {
        return if let Some(Fact::StringList(_, value)) = self.facts.get(key) {
            Some(&value)
        } else {
            None
        };
    }
}

#[derive(Resource, Deserialize, Serialize)]
pub struct RuleEngine {
    rules: HashMap<String, Rule>,
    rule_states: HashMap<String, bool>,
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
        self.rules
            .iter()
            .for_each(|(name, rule)| {
                let previous_state = self.rule_states.get(name).unwrap();
                if previous_state != &rule.evaluate(facts) {
                    self.rule_states.insert(name.clone(), !previous_state);
                    updated_rule_states.insert(name.clone());
                }
            });
        return updated_rule_states;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Condition {
    IntEquals { fact_name: String, expected_value: i32 },
    IntMoreThan { fact_name: String, expected_value: i32 },
    IntLessThan { fact_name: String, expected_value: i32 },
    StringEquals { fact_name: String, expected_value: String },
    BoolEquals { fact_name: String, expected_value: bool },
    ListContains { fact_name: String, expected_value: String },
}

impl Condition {
    // Evaluate the condition based on the provided facts
    pub fn evaluate(&self, facts: &HashMap<String, Fact>) -> bool {
        match self {
            Condition::IntEquals { fact_name, expected_value } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value == *expected_value;
                }
            }
            Condition::StringEquals { fact_name, expected_value } => {
                if let Some(Fact::String(_, value)) = facts.get(fact_name) {
                    return value == expected_value;
                }
            }
            Condition::BoolEquals { fact_name, expected_value } => {
                if let Some(Fact::Bool(_, value)) = facts.get(fact_name) {
                    return *value == *expected_value;
                }
            }
            Condition::IntMoreThan { fact_name, expected_value } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value > *expected_value;
                }
            }
            Condition::IntLessThan { fact_name, expected_value } => {
                if let Some(Fact::Int(_, value)) = facts.get(fact_name) {
                    return *value < *expected_value;
                }
            }
            Condition::ListContains { fact_name, expected_value } => {
                if let Some(Fact::StringList(_, value)) = facts.get(fact_name) {
                    return value.0.contains(expected_value);
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Rule {
    pub name: String,
    pub conditions: Vec<Condition>,
}

impl Rule {
    // Constructor for Rule
    pub fn new(name: String, conditions: Vec<Condition>) -> Self {
        Rule {
            name,
            conditions,
        }
    }

    // Evaluate all conditions for the rule based on the provided facts
    pub fn evaluate(&self, facts: &HashMap<String, Fact>) -> bool {
        self.conditions.iter().all(|condition| condition.evaluate(facts))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct StoryBeat {
    pub name: String,
    pub rules: Vec<Rule>,
    pub finished: bool,
}

impl StoryBeat {
    // Constructor for StoryBeat
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        StoryBeat {
            name,
            rules,
            finished: false,
        }
    }

    // Evaluate all rules for the story beat based on the provided facts
    pub fn evaluate(&mut self, facts: &HashMap<String, Fact>) {
        self.finished = self.rules.iter().all(|rule| rule.evaluate(facts));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Story {
    pub name: String,
    pub beats: Vec<StoryBeat>,
    pub active_beat_index: usize,
}

impl Story {
    // Constructor for Story
    pub fn new(name: String, beats: Vec<StoryBeat>) -> Self {
        Story {
            name,
            beats,
            active_beat_index: 0,
        }
    }

    // Evaluate the active story beat
    pub fn evaluate_active_beat(&mut self, facts: &HashMap<String, Fact>) {
        if self.active_beat_index < self.beats.len() {
            let active_beat = &mut self.beats[self.active_beat_index];
            active_beat.evaluate(facts);
            if active_beat.finished {
                self.active_beat_index += 1;
            }
        }
    }

    // Check if the story is finished
    pub fn is_finished(&self) -> bool {
        self.active_beat_index >= self.beats.len()
    }
}

#[derive(Resource,Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct StoryEngine {
    pub stories: Vec<Story>,
}

impl StoryEngine {
    // Constructor for StoryEngine
    pub fn new() -> Self {
        StoryEngine {
            stories: Vec::new(),
        }
    }

    // Add a story to the story engine
    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
    }

    // Evaluate all stories based on the provided facts
    pub fn evaluate_stories(&mut self, facts: &HashMap<String, Fact>) {
        for story in &mut self.stories {
            story.evaluate_active_beat(facts);
        }
    }

    // Check if all stories are finished
    pub fn all_stories_finished(&self) -> bool {
        self.stories.iter().all(|story| story.is_finished())
    }
}