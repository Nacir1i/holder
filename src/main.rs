mod assets_loader;
mod camera;
mod character;
mod debug;
mod plane;

use assets_loader::AssetsLoaderPlugin;
use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use camera::CameraPlugin;
use character::CharacterPlugin;
use leafwing_input_manager::prelude::*;
use plane::PlanePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(EditorPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<character::PlayerAction>::default())
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(PlanePlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GamepadPlugin)
        .run()
}

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamepad_connections);
    }
}

#[derive(Resource)]
struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for event in gamepad_evr.read() {
        match event {
            GamepadEvent::Connection(con) => {
                let gamepad = con.gamepad;
                match &con.connection {
                    GamepadConnection::Connected(gamepad_info) => {
                        println!(
                            "Gamepad with id: {:?} and name: {:?} was connected",
                            gamepad.id, gamepad_info.name
                        );
                        if my_gamepad.is_none() {
                            commands.insert_resource(MyGamepad(gamepad))
                        }
                    }
                    GamepadConnection::Disconnected => {
                        println!("Gamepad with id: {:?} was disconnected", gamepad.id);
                        // if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                        //     if old_id == gamepad {
                        //         commands.remove_resource::<MyGamepad>();
                        //     }
                        // }
                    }
                }
            }
            _ => {}
        }
    }
}
