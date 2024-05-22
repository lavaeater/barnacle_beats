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
pub struct StringHashSet(pub HashSet<String>);

impl StringHashSet {
    pub fn new() -> Self {
        StringHashSet(HashSet::new())
    }

    pub  fn insert(&mut self, value: String) -> bool {
        self.0.insert(value)
    }

    pub fn remove(&mut self, value: &String) -> bool {
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
pub struct FactsOfTheWorld {
    pub facts: HashMap<String, Fact>,
    pub updated_facts: HashSet<Fact>,
}

impl FactsOfTheWorld {
    pub fn new() -> Self {
        FactsOfTheWorld {
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
    pub effects: Vec<Effect>,
    pub finished: bool,
}

impl StoryBeat {
    // Constructor for StoryBeat
    pub fn new(name: String, rules: Vec<Rule>, effects: Vec<Effect>) -> Self {
        StoryBeat {
            name,
            rules,
            effects,
            finished: false,
        }
    }

    // Evaluate all rules for the story beat based on the provided facts
    pub fn evaluate(&mut self, facts: &HashMap<String, Fact>) {
        self.finished = self.rules.iter().all(|rule| rule.evaluate(facts));
    }
}

// Story struct
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Story {
    pub name: String,
    pub pre_requisites: Vec<Rule>,
    pub beats: Vec<StoryBeat>,
    pub is_started: bool,
    pub active_beat_index: usize,
}

impl Story {
    pub fn new(name: String, pre_requisites: Vec<Rule>, beats: Vec<StoryBeat>) -> Self {
        Story {
            name,
            pre_requisites,
            beats,
            is_started: false,
            active_beat_index: 0,
        }
    }

    pub fn evaluate_active_beat(&mut self, facts: &HashMap<String, Fact>) -> Option<StoryBeat> {
        if self.active_beat_index < self.beats.len() {
            let active_beat = &mut self.beats[self.active_beat_index];
            active_beat.evaluate(facts);
            if active_beat.finished {
                self.active_beat_index += 1;
                Some(active_beat.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn start_if_possible(&mut self, facts: &HashMap<String, Fact>) -> bool {
        if !self.is_started {
            self.is_started = self.pre_requisites.iter().all(|rule| rule.evaluate(facts));
        }
        self.is_started
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

    // Check if all stories are finished
    pub fn all_stories_finished(&self) -> bool {
        self.stories.iter().all(|story| story.is_finished())
    }
}

#[derive(Event)]
pub struct StoryBeatFinished {
    pub story: Story,
    pub beat: StoryBeat,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Effect {
    SetFact(Fact),
}

impl Effect {
    pub fn apply(&self, fact_store: &mut FactsOfTheWorld) {
        match self {
            Effect::SetFact(fact) => {
                match fact {
                    Fact::Int(name, value) => fact_store.store_int(name.clone(), *value),
                    Fact::String(name, value) => fact_store.store_string(name.clone(), value.clone()),
                    Fact::Bool(name, value) => fact_store.store_bool(name.clone(), *value),
                    Fact::StringList(name, values) => {
                        for value in &values.0 {
                            fact_store.add_to_list(name.clone(), value.clone());
                        }
                    },
                }
            }
        }
    }
}




