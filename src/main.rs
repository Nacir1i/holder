mod assets_loader;
mod camera;
mod character;
mod debug;
mod plane;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use leafwing_input_manager::prelude::*;

use assets_loader::AssetsLoaderPlugin;
use camera::CameraPlugin;
use character::CharacterPlugin;
use plane::PlanePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(ui::fps_counter::FpsCounterPlugin)
        .add_plugins(EditorPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<character::PlayerAction>::default())
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(PlanePlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(CameraPlugin)
        .run()
}
