use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::assets_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPEED: f32 = 5.0;
const ROTATION_SPEED: f32 = 2.5;

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
    let mut input_map = InputMap::default();

    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
        input_map.insert(PlayerAction::default_gamepad_input(action), action);
    }

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.character.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        InputManagerBundle::<PlayerAction> {
            input_map,
            ..default()
        },
        Character,
    ));
}

fn character_movement_controls(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut ActionState<PlayerAction>,
        ),
        With<Character>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, input) = query.single_mut();

    let mut rotation = 0.0;
    let mut movement = 0.0;

    if input.pressed(PlayerAction::Run) {
        println!("Running");
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement = SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement = -SPEED;
    }

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    velocity.value = -transform.forward() * movement;
}
