mod camera;
mod character;
mod debug;
mod ground;
mod light;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    Main,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "models/character.glb#Scene0")]
    pub character: Handle<Scene>,

    #[asset(path = "terrains/room.glb#Scene0")]
    pub room: Handle<Scene>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Main)
                .load_collection::<GameAssets>(),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(ui::fps_counter::FpsCounterPlugin)
        .add_plugins(EditorPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(InputManagerPlugin::<character::PlayerAction>::default())
        //User defined plugins
        .add_plugins(light::LightPlugin)
        .add_plugins(ground::GroundPlugin)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(camera::CameraPlugin)
        .run()
}
