use bevy::utils::HashSet;
use crate::beats::data::{Effect, Fact, Rule, Story, StoryBeat, StringHashSet};

#[derive(Debug, Default)]
pub struct EffectBuilder {
    effects: Vec<Effect>,
}

impl EffectBuilder {
    pub fn new() -> Self {
        EffectBuilder {
            effects: Vec::new(),
        }
    }

    pub fn set_fact_int(mut self, name: impl Into<String>, value: i32) -> Self {
        self.effects.push(Effect::SetFact(Fact::Int(name.into(), value)));
        self
    }

    pub fn set_fact_string(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.effects.push(Effect::SetFact(Fact::String(name.into(), value.into())));
        self
    }

    pub fn set_fact_bool(mut self, name: impl Into<String>, value: bool) -> Self {
        self.effects.push(Effect::SetFact(Fact::Bool(name.into(), value)));
        self
    }

    pub fn set_fact_string_list(mut self, name: impl Into<String>, values: HashSet<String>) -> Self {
        self.effects.push(Effect::SetFact(Fact::StringList(name.into(), StringHashSet(values))));
        self
    }

    pub fn build(self) -> Vec<Effect> {
        self.effects
    }
}

#[derive(Debug, Default)]
pub struct StoryBeatBuilder {
    name: String,
    rules: Vec<Rule>,
    effects: Vec<Effect>,
}

impl StoryBeatBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        StoryBeatBuilder {
            name: name.into(),
            rules: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn add_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn with_effects<F>(mut self, build_fn: F) -> Self
        where
            F: FnOnce(EffectBuilder) -> EffectBuilder,
    {
        let builder = EffectBuilder::new();
        let effects = build_fn(builder).build();
        self.effects.extend(effects);
        self
    }

    pub fn build(self) -> StoryBeat {
        StoryBeat {
            name: self.name,
            rules: self.rules,
            effects: self.effects,
            finished: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct StoryBuilder {
    name: String,
    beats: Vec<StoryBeat>,
}

impl StoryBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        StoryBuilder {
            name: name.into(),
            beats: Vec::new(),
        }
    }

    pub fn add_story_beat<F>(mut self, name: impl Into<String>, build_fn: F) -> Self
        where
            F: FnOnce(StoryBeatBuilder) -> StoryBeatBuilder,
    {
        let builder = StoryBeatBuilder::new(name.into());
        let beat = build_fn(builder).build();
        self.beats.push(beat);
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
