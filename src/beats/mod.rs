use crate::beats::data::*;
use crate::beats::systems::*;
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Component, IntoSystemConfigs, OnEnter};

pub mod data;
mod parsing;
pub mod systems;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoolFactStore::new())
            .insert_resource(RuleEngine::new())
            .insert_resource(StoryEngine::new())
            .add_event::<FactUpdated>()
            .add_event::<RuleUpdated>()
            .add_event::<StoryBeatFinished>()
            .add_systems(
                OnEnter(GameState::Story),
                (setup, spawn_layout, setup_rules, setup_stories),
            )
            .add_systems(
                Update,
                (
                    fact_update_event_broadcaster,
                    fact_event_system,
                    rule_event_system,
                    rule_evaluator,
                    button_system,
                    story_evaluator,
                    story_beat_effect_applier
                )
                    .run_if(in_state(GameState::Story)),
            );
    }
}
#[derive(Component)]
pub struct TextComponent;
