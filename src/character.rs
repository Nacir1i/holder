use bevy::prelude::*;

use crate::assets_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPEED: f32 = 5.0;
const ROTATION_SPEED: f32 = 2.5;

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
        Character,
    ));
}

fn character_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Character>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    let mut rotation = 0.0;
    let mut movement = 0.0;

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
