use crate::beats::data::{Condition, FactsOfTheWorld, FactUpdated, Rule, RuleUpdated, StoryBeatFinished, StoryEngine};
use crate::beats::TextComponent;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::hierarchy::{ChildBuilder, Children};
use bevy::math::Vec2;
use bevy::prelude::{default, AlignItems, BackgroundColor, BorderColor, BuildChildren, Button, ButtonBundle, Changed, Color, ColorMaterial, Commands, Display, EventReader, EventWriter, Font, GridPlacement, GridTrack, Interaction, JustifyContent, JustifyItems, Mesh, NodeBundle, PositionType, Query, RepeatedGridTrack, Res, ResMut, Style, Text, TextBundle, TextStyle, Transform, Triangle2d, UiRect, Val, Visibility, With, JustifyText};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::beats::builders::StoryBuilder;
use crate::ui::builders::{add_button, NodeBundleBuilder};

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn(
            NodeBundleBuilder::new()
                .with_style(|style_builder| {
                    style_builder
                        .with_grid()
                        .width_and_height_in_percent(100.0, 100.0)
                        .grid_template_columns(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
                        .grid_template_rows(vec![
                            GridTrack::auto(),
                            GridTrack::flex(1.0),
                            GridTrack::px(20.),
                        ])
                })
                .with_background_color(Color::BLACK)
                .build()
        )
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundleBuilder::new()
                    .with_style(|style_builder| {
                        style_builder
                            .with_grid()
                            .span_columns(2)
                            .pad_all_px(12.0)
                    })
                    .with_background_color(Color::BLACK)
                    .build()
                )
                .with_children(|builder| {
                    text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example", 24.0, Color::BLACK);
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(
                    NodeBundleBuilder::new()
                        .with_style(|style_builder| {
                            style_builder
                                .with_grid()
                                .fill_parent_height()
                                .aspect_ratio(1.0)
                                .pad_all_px(24.0)
                                .flex_columns(4, 1.0)
                                .flex_rows(4, 1.0)
                                .gutter_all_px(6.0)
                        })
                        .with_background_color(Color::DARK_GRAY)
                        .build()
                )
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    item_rect(builder, Color::ORANGE, false, font.clone_weak());
                    item_rect(builder, Color::BISQUE, false, font.clone_weak());
                    item_rect(builder, Color::BLUE, false, font.clone_weak());
                    item_rect(builder, Color::CRIMSON, false, font.clone_weak());

                    item_rect(builder, Color::CYAN, false, font.clone_weak());
                    item_rect(builder, Color::ORANGE_RED, false, font.clone_weak());
                    item_rect(builder, Color::DARK_GREEN, false, font.clone_weak());
                    item_rect(builder, Color::FUCHSIA, false, font.clone_weak());

                    item_rect(builder, Color::TEAL, false, font.clone_weak());
                    item_rect(builder, Color::ALICE_BLUE, false, font.clone_weak());
                    item_rect(builder, Color::CRIMSON, false, font.clone_weak());
                    item_rect(builder, Color::ANTIQUE_WHITE, false, font.clone_weak());

                    item_rect(builder, Color::YELLOW, false, font.clone_weak());
                    item_rect(builder, Color::PINK, false, font.clone_weak());
                    item_rect(builder, Color::YELLOW_GREEN, false, font.clone_weak());
                    item_rect(builder, Color::SALMON, true, font.clone_weak());
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Align content towards the start (top) in the vertical axis
                        align_items: AlignItems::Start,
                        // Align content towards the center in the horizontal axis
                        justify_items: JustifyItems::Center,
                        // Add 10px padding
                        padding: UiRect::all(Val::Px(10.)),
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                        // Add a 10px gap between rows
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Sidebar",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ));
                    builder.spawn((TextBundle::from_section(
                        "A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                    ), TextComponent
                    ));
                    builder.spawn(NodeBundle::default());
                });

            // Footer / status bar
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });

            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
            builder.spawn(NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(100.),
                        bottom: Val::Auto,
                        left: Val::Auto,
                        right: Val::Auto,
                    },
                    width: Val::Percent(60.),
                    height: Val::Px(300.),
                    max_width: Val::Px(600.),
                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 255.0,
                    green: 255.0,
                    blue: 255.0,
                    alpha: 0.8,
                }),
                ..default()
            });
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
pub fn item_rect(builder: &mut ChildBuilder, color: Color, with_button: bool, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            if with_button {
                add_button(builder, "Press", font.clone(), 24.0, Color::WHITE, color, Color::WHITE, |style_builder| {
                    style_builder
                        .with_grid()
                        .width_and_height_in_percent(100.0, 100.0)
                        .centered_content()
                })   
            }

            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str, font_size: f32, color: Color) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size,
            color,
        },
    )
        .with_text_justify(JustifyText::Center)
    );
}

pub(crate) const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn fact_event_system(
    mut query: Query<&mut Text, With<TextComponent>>,
    mut fact_update_events: EventReader<FactUpdated>,
    mut story_beat_updated: EventReader<StoryBeatFinished>,
) {
    for event in fact_update_events.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}\n Fact Updated: {:?}\n", text.sections[0].value, event.fact);
        }
    }

    for story_updated in story_beat_updated.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}\n Story Beat updated: {:?}\n", text.sections[0].value, story_updated.beat.name);
        }
    }
}

pub fn fact_update_event_broadcaster(
    mut event_writer: EventWriter<FactUpdated>,
    mut storage: ResMut<FactsOfTheWorld>,
) {
    for fact in storage.updated_facts.drain() {
        event_writer.send(FactUpdated { fact });
    }
}

pub fn rule_event_system(
    mut query: Query<&mut Text, With<TextComponent>>,
    mut rule_updated_events: EventReader<RuleUpdated>,
) {
    for event in rule_updated_events.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}\n{:?}", text.sections[0].value, event.rule);
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut storage: ResMut<FactsOfTheWorld>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                storage.add_to_int("button_pressed".to_string(), 1);
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value =
                    storage.get_int("button_pressed").unwrap_or(&0).to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Press to add".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands.spawn(Camera2dBundle::default());

    let shapes = [Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 50.0,
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    )))];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -crate::beats::data::X_EXTENT / 2.
                    + i as f32 / num_shapes as f32 * crate::beats::data::X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}

pub fn story_evaluator(
    mut fact_updated: EventReader<FactUpdated>,
    mut story_engine: ResMut<StoryEngine>,
    cool_fact_store: Res<FactsOfTheWorld>,
    mut story_beat_writer: EventWriter<StoryBeatFinished>,
) {
    if !fact_updated.is_empty() {
        fact_updated.clear();
        for story in &mut story_engine.stories.iter_mut().filter(|s| !s.is_started) {
            story.start_if_possible(&cool_fact_store.facts);
        }

        for story in &mut story_engine.stories.iter_mut().filter(|s| s.is_started && !s.is_finished()) {
            match story.evaluate_active_beat(&cool_fact_store.facts) {
                None => {}
                Some(story_beat) => {
                    story_beat_writer.send(StoryBeatFinished {
                        story: story.clone(),
                        beat: story_beat.clone(),
                    });
                }
            }
        }
    }
}

pub fn story_beat_effect_applier(
    mut story_beat_reader: EventReader<StoryBeatFinished>,
    mut cool_fact_store: ResMut<FactsOfTheWorld>,
) {
    for event in story_beat_reader.read() {
        for effect in event.beat.effects.iter() {
            effect.apply(&mut cool_fact_store);
        }
    }
}

pub fn setup_stories(
    mut story_engine: ResMut<StoryEngine>,
) {
    /*
    Let's imagine two stories. One that simply requires that the button is pressed three times.
    When pressed three times, some kind of message needs to be displayed.
    In fact, to make all this as loosely connected as possible, we always work with facts / events.
    I think every story beat should have some kind of list of consequences to be applied when done.

    This could be a simple case of enum variants to be used for this.

     */
    let story = StoryBuilder::new("Hero's Journey")
        .add_pre_requisite("Before We Start", |pre_req| {
            pre_req.with_condition(Condition::IntMoreThan {
                fact_name: "button_pressed".to_string(),
                expected_value: 1,
            })
        })
        .add_story_beat("The Call to Adventure", |beat| {
            beat.with_rule("Enough Presses", |rule| {
                rule.with_condition(Condition::IntMoreThan {
                    fact_name: "button_pressed".to_string(),
                    expected_value: 3,
                })
            })
                .with_effects(|effects| {
                    effects.set_fact_bool("quest_one_complete", true)
                })
        })
        .add_story_beat("The Road of Trials", |beat| {
            beat.with_rule("DefeatedEnemies", |rule| {
                rule.with_condition(Condition::IntMoreThan {
                    fact_name: "button_pressed".to_string(),
                    expected_value: 5,
                })
            })
                .with_effects(|effects| {
                    effects.set_fact_bool("quest_two_complete", true)
                })
        })
        .build();

    story_engine.add_story(story);
}
