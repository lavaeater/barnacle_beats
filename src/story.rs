use std::hash::{Hash, Hasher};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
const X_EXTENT: f32 = 600.;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CoolFactStore::new())
            .insert_resource(RuleEngine::new())
            .insert_resource(StoryEngine::new())
            .add_event::<FactUpdated>()
            .add_event::<RuleUpdated>()
            .add_systems(OnEnter(GameState::Story), (
                setup,
                spawn_layout,
                setup_rules))
            .add_systems(Update, (
                fact_update_event_broadcaster,
                fact_event_system,
                rule_event_system,
                rule_evaluator)
                .run_if(in_state(GameState::Story)));
    }
}


#[derive(Event)]
pub struct FactUpdated {
    fact: Fact,
}

#[derive(Event)]
pub struct RuleUpdated {
    rule: String,
}

fn fact_update_event_broadcaster(
    mut event_writer: EventWriter<FactUpdated>,
    mut storage: ResMut<CoolFactStore>,
) {
    for fact in storage.updated_facts.drain() {
        event_writer.send(FactUpdated {
            fact
        });
    }
}

#[derive(Component)]
pub struct TextComponent;

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
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
fn item_rect(builder: &mut ChildBuilder, color: Color, with_button: bool, font: Handle<Font>) {
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
                builder.spawn(ButtonBundle {
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

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
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

fn fact_event_system(
    mut query: Query<&mut Text, With<TextComponent>>,
    mut fact_update_events: EventReader<FactUpdated>,
) {
    for event in fact_update_events.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}\n{:?}", text.sections[0].value, event.fact);
        }
    }
}

fn rule_event_system(
    mut query: Query<&mut Text, With<TextComponent>>,
    mut rule_updated_events: EventReader<RuleUpdated>,
) {
    for event in rule_updated_events.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}\n{:?}", text.sections[0].value, event.rule);
        }
    }
}


fn button_system(
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
    mut storage: ResMut<CoolFactStore>,
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
                text.sections[0].value = storage.get_int("button_pressed").unwrap_or(&0).to_string();
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


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}

#[derive(Resource, Deserialize, Serialize)]
struct CoolFactStore {
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
    fn store_int(&mut self, key: String, value: i32) {
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

    fn add_to_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    fn subtract_from_int(&mut self, key: String, value: i32) {
        let current = self.get_int(&key).unwrap_or(&0);
        self.store_int(key, current + value);
    }

    // Store a string fact
    fn store_string(&mut self, key: String, value: String) {
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
    fn store_bool(&mut self, key: String, value: bool) {
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
    fn get_int(&self, key: &str) -> Option<&i32> {
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

fn setup_rules(
    mut rule_engine: ResMut<RuleEngine>,
) {
    let rule1 = Rule::new(
        "button_pressed_rule".to_string(),
        vec![
            Condition::IntMoreThan { fact_name: "button_pressed".to_string(), expected_value: 5 },
        ],
    );

    rule_engine.add_rule(rule1);
}

fn rule_evaluator(
    mut rules: ResMut<RuleEngine>,
    mut fact_updated: EventReader<FactUpdated>,
    mut rule_updated_writer: EventWriter<RuleUpdated>,
    storage: Res<CoolFactStore>,
) {
    // we obviously only update when facts are updated. In future, only update rules
    // that are affected by the updated facts
    for _ in fact_updated.read() {
        let facts = &storage.facts;
        let results = rules.evaluate_rules(facts);
        for rule_name in results {
            rule_updated_writer.send(RuleUpdated {
                rule: rule_name.clone()
            });
        }
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

fn setup_stories(
    mut story_engine: ResMut<StoryEngine>,
    mut cool_fact_store: ResMut<CoolFactStore>,
) {
    cool_fact_store.store_int("age".to_string(), 25);
    cool_fact_store.store_string("name".to_string(), "John".to_string());
    cool_fact_store.store_bool("has_car".to_string(), true);

    // Define some rules
    let rule1 = Rule::new(
        "rule1".to_string(),
        vec![
            Condition::IntEquals { fact_name: "age".to_string(), expected_value: 25 },
        ],
    );

    let rule2 = Rule::new(
        "rule2".to_string(),
        vec![
            Condition::StringEquals { fact_name: "name".to_string(), expected_value: "John".to_string() },
        ],
    );

    let rule3 = Rule::new(
        "rule3".to_string(),
        vec![
            Condition::BoolEquals { fact_name: "has_car".to_string(), expected_value: true },
        ],
    );

    // Define some story beats
    let beat1 = StoryBeat::new(
        "beat1".to_string(),
        vec![rule1],
    );

    let beat2 = StoryBeat::new(
        "beat2".to_string(),
        vec![rule2],
    );

    let beat3 = StoryBeat::new(
        "beat3".to_string(),
        vec![rule3],
    );

    // Define a story with the beats
    let story = Story::new(
        "story1".to_string(),
        vec![beat1, beat2, beat3],
    );

    story_engine.add_story(story);
}