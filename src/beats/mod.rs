use crate::beats::data::*;
use crate::beats::systems::*;
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Component, IntoSystemConfigs, OnEnter, Commands, not, any_with_component, Query, Entity, With, Res, Time, PositionType, Val, Color};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::ui::fps_widget;
use sickle_ui::{
    ui_builder::{UiBuilderExt, UiRoot},
    ui_commands::SetTextExt,
    ui_style::{
        SetBackgroundColorExt, SetNodeHeightExt, SetNodePositionTypeExt, SetNodeRightExt,
        SetNodeTopExt,
    },
    widgets::{
        column::UiColumnExt,
        label::{LabelConfig, UiLabelExt},
    },
};
use crate::ui::banner_widget::{BannerWidget, BannerWidgetCommands, BannerWidgetConfig, UiBannerWidgetExt};
use crate::ui::fps_widget::{FpsWidget, UiFPSWidgetExt};

pub mod data;
pub mod systems;
mod builders;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FactsOfTheWorld::new())
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(fps_widget::plugin)
            .insert_resource(StoryEngine::new())
            .add_event::<FactUpdated>()
            .add_event::<RuleUpdated>()
            .add_event::<StoryBeatFinished>()
            .add_systems(
                OnEnter(GameState::Story),
                (setup_stories), //setup, spawn_layout, 
            )
            .add_systems(
                Update,
                (
                    fact_update_event_broadcaster,
                    fact_event_system,
                    rule_event_system,
                    button_system,
                    story_evaluator,
                    story_beat_effect_applier
                )
                    .run_if(in_state(GameState::Story)),
            )
            .add_systems(
                Update,
                (
                    spawn_simple_widget.run_if(not(any_with_component::<SimpleWidget>)),
                    spawn_fps_widget.run_if(not(any_with_component::<FpsWidget>)),
                    spawn_banner_widgets.run_if(not(any_with_component::<BannerWidget>)),
                    move_banner_example,
                ).run_if(in_state(GameState::Story)))
        ;
    }
}
#[derive(Component)]
pub struct TextComponent;


#[derive(Component)]
struct SimpleWidget;

fn spawn_simple_widget(mut commands: Commands) {
    // Let's create a simple column widget on the screen.
    commands.ui_builder(UiRoot).column(|column| {
        column.entity_commands().insert(SimpleWidget);

        // We can style our widget directly in code using the style method.
        column
            .style()
            // The column will be located 100 pixels from the right and 100 pixels from the top of the screen.
            // The absolute position means we are not set relative to any parent.
            .position_type(PositionType::Absolute)
            .right(Val::Px(100.0))
            .top(Val::Px(100.0))
            // We'll bound the height of our column to the total height of our contents.
            // By default, a column will be 100% of the parent's height which would be the entire length of the screen.,
            .height(Val::Auto)
            // Lets give it a visible background color.
            .background_color(Color::rgb(0.5, 0.5, 0.5));

        // Let's add some content to our column.
        column
            .label(LabelConfig::default())
            .entity_commands()
            // We can use the set_text method to set the text of a label.
            .set_text("This is label 1.", None);

        column
            .label(LabelConfig::default())
            .entity_commands()
            .set_text("This is another label.", None);
    });
}

fn spawn_fps_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).fps();
}

#[derive(Component)]
struct FlyingExample;

fn spawn_banner_widgets(mut commands: Commands) {
    let font = "FiraSans-Bold.ttf";
    let font_size = 30.0;

    commands
        .ui_builder(UiRoot)
        .banner_widget(BannerWidgetConfig::new("Hello, World!", font, font_size))
        .entity_commands()
        .set_position(100.0, 100.0);

    commands
        .ui_builder(UiRoot)
        .banner_widget(BannerWidgetConfig::new(
            "Bonjour, le Monde!",
            font,
            font_size,
        ))
        .entity_commands()
        .set_position(300.0, 300.0);

    commands
        .ui_builder(UiRoot)
        .banner_widget(BannerWidgetConfig::new("¡Hola, Mundo!", font, font_size))
        .entity_commands()
        .set_position(700.0, 100.0)
        .insert(FlyingExample);
}

fn move_banner_example(
    mut commands: Commands,
    examples: Query<Entity, With<FlyingExample>>,
    time: Res<Time>,
) {
    for entity in examples.iter() {
        commands.entity(entity).set_position(
            700.0 + time.elapsed_seconds().sin() * 100.0,
            100.0 + time.elapsed_seconds().cos() * 100.0,
        );
    }
}