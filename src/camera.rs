use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;

const CAMERA_DISTANCE: f32 = 2.5;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, CAMERA_DISTANCE, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            mouse_sensitivity: 2.5,
            ..default()
        },
    ));
}
