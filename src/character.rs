use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::assets_loader::SceneAssets;
use crate::plane::Gravity;

const SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
const STARTING_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);
const STARTING_TRANSLATION: Transform = Transform {
    translation: STARTING_POSITION,
    scale: SCALE,
    rotation: Quat::IDENTITY,
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Run,
    Jump,
    UseItem,
}

impl PlayerAction {
    fn default_keyboard_mouse_input(action: PlayerAction) -> UserInput {
        match action {
            Self::Run => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Self::Jump => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            Self::UseItem => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
        }
    }

    fn default_gamepad_input(action: PlayerAction) -> UserInput {
        match action {
            Self::Run => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Self::Jump => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
            Self::UseItem => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_character);
        app.add_systems(Update, character_movement_controls);
    }
}

#[derive(Component)]
pub struct Character;

fn spawn_character(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let init_transform = TransformBundle::from_transform(STARTING_TRANSLATION);
    let mut input_map = InputMap::default();

    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
        input_map.insert(PlayerAction::default_gamepad_input(action), action);
    }

    commands.spawn((
        SceneBundle {
            scene: scene_assets.character.clone(),
            transform: init_transform.local,
            global_transform: init_transform.global,
            ..default()
        },
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        Collider::cylinder(1., 0.2),
        Restitution::coefficient(0.7),
        InputManagerBundle::<PlayerAction> {
            input_map,
            ..default()
        },
        ThirdPersonCameraTarget,
        Character,
        Gravity,
    ));
}

fn character_movement_controls() {}
