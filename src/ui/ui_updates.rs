use bevy::prelude::{Changed, Component, Query, Resource, Text, With};
use bevy::utils::HashMap;

#[derive(Component)]
pub struct UiText {
    pub value: String,
}

#[derive(Resource)]
pub struct UiTexts {
    pub texts: HashMap<String, UiText>,
}

impl UiTexts {
    fn new_text(&mut self, key: impl Into<String>, value: impl Into<String>) -> UiText {
        let ui_text = UiText::new(value);
        self.texts.insert(key.into(), ui_text);
        ui_text
    }
}

impl UiText {
    pub fn new(value: impl Into<String>) -> Self {
        UiText {
            value: value.into(),
        }
    }
}

pub fn text_update_system(
    mut query: Query<(&mut Text, &UiText), Changed<UiText>>) {
    for (mut text, ui_text) in query.iter_mut() {
        text.sections[0].value.clone_from(&ui_text.value);
    }

    // query.
    //
    // for (text, ui_text) in query.iter_mut() {
    //     if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    //         if let Some(value) = fps.smoothed() {
    //             // Update the value of the second section
    //             text.sections[1].value = format!("{value:.2}");
    //         }
    //     }
    // }
}