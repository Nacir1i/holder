use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, -1.0, 0.0);

#[derive(Component)]
pub struct Gravity;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_plane)
            .add_systems(Update, apply_gravity);
    }
}

fn spawn_plane(mut commands: Commands) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.1, 100.0),
        TransformBundle::from(Transform::from_translation(STARTING_TRANSLATION)),
    ));
}

fn apply_gravity(mut controllers: Query<&mut KinematicCharacterController, With<Gravity>>) {
    for mut controller in &mut controllers {
        controller.translation = Some(Vec3::new(0., -0.1, 0.));
    }
}

// fn print_entity_movement(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
//     for (entity, output) in &controllers {
//         info!(
//             "Entity {:?} moved by {:?} and touches the ground: {:?}",
//             entity, output.effective_translation, output.grounded
//         );
//     }
// }
