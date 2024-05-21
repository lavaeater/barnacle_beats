use crate::beats::data::{Condition, CoolFactStore, FactUpdated, Rule, RuleEngine, RuleUpdated, Story, StoryBeat, StoryBeatFinished, StoryEngine};
use crate::beats::TextComponent;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::hierarchy::{ChildBuilder, Children};
use bevy::math::Vec2;
use bevy::prelude::{default, AlignItems, BackgroundColor, BorderColor, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, ColorMaterial, Commands, Display, EventReader, EventWriter, Font, GridPlacement, GridTrack, Interaction, JustifyContent, JustifyItems, Mesh, NodeBundle, PositionType, Query, RepeatedGridTrack, Res, ResMut, Style, Text, TextBundle, TextStyle, Transform, Triangle2d, UiRect, Val, Visibility, With, Local, Time};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use nom::combinator::all_consuming;
use crate::beats::parsing::parse_story;

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of it's contents
                //   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of it's contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Make this node span two grid columns so that it takes up the entire top tow
                        grid_column: GridPlacement::span(2),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        // Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        // As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        // Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
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
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font,
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }

            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn fact_event_system(
    mut query: Query<&mut Text, With<TextComponent>>,
    mut fact_update_events: EventReader<FactUpdated>,
    mut story_beat_updated: EventReader<StoryBeatFinished>
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
    mut storage: ResMut<CoolFactStore>,
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
    mut storage: ResMut<crate::beats::data::CoolFactStore>,
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
                    + i as f32 / (num_shapes) as f32 * crate::beats::data::X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}

pub fn setup_rules(mut rule_engine: ResMut<RuleEngine>) {
    let rule1 = Rule::new(
        "button_pressed_rule".to_string(),
        vec![Condition::IntMoreThan {
            fact_name: "button_pressed".to_string(),
            expected_value: 5,
        }],
    );

    rule_engine.add_rule(rule1);
}

pub fn story_evaluator(
    mut fact_updated: EventReader<FactUpdated>,
    mut story_engine: ResMut<StoryEngine>,
    cool_fact_store: Res<CoolFactStore>,
    mut story_beat_writer: EventWriter<StoryBeatFinished>,
) {
    if !fact_updated.is_empty() {
        fact_updated.clear();
        for story in &mut story_engine.stories {
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
    mut cool_fact_store: ResMut<CoolFactStore>,
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
    let input = r#"
# Story: First Story

## StoryBeat: The story begins
- Rule: Start Rule
    - Condition: IntMoreThan(button_pressed, 3)
- Effect: SetFact Bool beat_one true

## StoryBeat: SecondBeat
- Rule: Start Second Rule
    - Condition: IntMoreThan(buttom_pressed, 10)
- Effect: SetFact Bool beat_two true
"#;

    match all_consuming(parse_story)(input) {
        Ok((_, story)) => story_engine.add_story(story),
        Err(e) => eprintln!("Error parsing story: {:?}", e),
    }

    //
    //
    // // Define some rules
    // let rule1 = Rule::new(
    //     "rule1".to_string(),
    //     vec![Condition::IntEquals {
    //         fact_name: "age".to_string(),
    //         expected_value: 25,
    //     }],
    // );
    //
    // let rule2 = Rule::new(
    //     "rule2".to_string(),
    //     vec![Condition::StringEquals {
    //         fact_name: "name".to_string(),
    //         expected_value: "John".to_string(),
    //     }],
    // );
    //
    // let rule3 = Rule::new(
    //     "rule3".to_string(),
    //     vec![Condition::BoolEquals {
    //         fact_name: "has_car".to_string(),
    //         expected_value: true,
    //     }],
    // );
    //
    // // Define some story beats
    // let beat1 = StoryBeat::new("beat1".to_string(), vec![rule1]);
    //
    // let beat2 = StoryBeat::new("beat2".to_string(), vec![rule2]);
    //
    // let beat3 = StoryBeat::new("beat3".to_string(), vec![rule3]);
    //
    // // Define a story with the beats
    // let story = Story::new("story1".to_string(), vec![beat1, beat2, beat3]);
}
