// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use barnacle_beats::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use std::io::Cursor;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Barnacle Beats".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        // .add_systems(Startup, set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    // let icon_buf = Cursor::new(include_bytes!(
    //     "../build/macos/AppIcon.iconset/icon_256x256.png"
    // ));
    // if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
    //     let image = image.into_rgba8();
    //     let (width, height) = image.dimensions();
    //     let rgba = image.into_raw();
    //     let icon = Icon::from_rgba(rgba, width, height).unwrap();
    //     primary.set_window_icon(Some(icon));
    // };
}
