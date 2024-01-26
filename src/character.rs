use bevy::{ecs::query::Has, prelude::*};
use bevy_xpbd_3d::{math::*, prelude::*, SubstepSchedule, SubstepSet};
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::{AppState, GameAssets};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 10.0, 0.0);

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Main), spawn_character)
            .add_systems(
                Update,
                (
                    update_grounded,
                    apply_deferred,
                    apply_gravity,
                    player_actions,
                    apply_movement_damping,
                )
                    .run_if(in_state(AppState::Main))
                    .chain(),
            )
            .add_systems(
                SubstepSchedule,
                kinematic_controller_collisions.in_set(SubstepSet::SolveUserConstraints),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Run,
    Jump,
    UseItem,
}

impl PlayerAction {
    fn default_keyboard_mouse_input(action: PlayerAction) -> UserInput {
        // Match against the provided action to get the correct default keyboard-mouse input
        match action {
            Self::Run => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Self::Jump => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            Self::UseItem => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
        }
    }

    fn default_gamepad_input(action: PlayerAction) -> UserInput {
        // Match against the provided action to get the correct default gamepad input
        match action {
            Self::Run => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Self::Jump => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
            Self::UseItem => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
        }
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Vector::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

fn spawn_character(mut commands: Commands, scene_assets: Res<GameAssets>) {
    let mut input_map = InputMap::default();

    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
        input_map.insert(PlayerAction::default_gamepad_input(action), action);
    }

    commands.spawn((
        SceneBundle {
            scene: scene_assets.character.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(1.25, 0.2), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
        InputManagerBundle::<PlayerAction> {
            input_map,
            ..default()
        },
        DebugRender::default().with_collider_color(Color::RED),
    ));
}

fn player_actions(
    time: Res<Time>,
    action_q: Query<&ActionState<PlayerAction>, With<CharacterController>>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        &mut Transform,
        Has<Grounded>,
    )>,
) {
    let action_state = action_q.single();

    for (movement_acceleration, jump_impulse, mut linear_velocity, mut transform, is_grounded) in
        &mut controllers
    {
        if action_state.pressed(PlayerAction::Run) {
            let direction = action_state
                .clamped_axis_pair(PlayerAction::Run)
                .unwrap()
                .xy();

            let rotation_angle = if direction.length() > 0.0 {
                direction.x.atan2(direction.y)
            } else {
                0.0
            };

            transform.rotation = Quat::from_rotation_y(-rotation_angle as f32);

            if is_grounded {
                linear_velocity.x += direction.x * movement_acceleration.0 * time.delta_seconds();
                linear_velocity.z -= direction.y * movement_acceleration.0 * time.delta_seconds();
            } else {
                linear_velocity.x +=
                    direction.x * (movement_acceleration.0 * 0.5) * time.delta_seconds();
                linear_velocity.z -=
                    direction.y * (movement_acceleration.0 * 0.5) * time.delta_seconds();
            }
        }

        if action_state.just_pressed(PlayerAction::Jump) {
            if is_grounded {
                linear_velocity.y = jump_impulse.0;
            }
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&ControllerGravity, &mut LinearVelocity)>,
) {
    for (gravity, mut linear_velocity) in &mut controllers {
        linear_velocity.0 += gravity.0 * time.delta_seconds();
    }
}

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system performs very basic collision response for kinematic
/// character controllers by pushing them along their contact normals
/// by the current penetration depths.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &RigidBody,
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        With<CharacterController>,
    >,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // If the collision didn't happen during this substep, skip the collision
        if !contacts.during_current_substep {
            continue;
        }

        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;
        let (rb, mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers
        if !rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            // Solve each penetrating contact in the manifold
            for contact in manifold.contacts.iter().filter(|c| c.penetration > 0.0) {
                position.0 += normal * contact.penetration;
            }

            // If the slope isn't too steep to walk on but the character
            // is falling, reset vertical velocity.
            if max_slope_angle.is_some_and(|angle| normal.angle_between(Vector::Y).abs() <= angle.0)
                && linear_velocity.y < 0.0
            {
                linear_velocity.y = linear_velocity.y.max(0.0);
            }
        }
    }
}
