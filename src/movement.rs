use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Acceleration {
        Acceleration { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}

impl Plugin for MovingObjectBundle {
    fn build(&self, app: &mut App) {}
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {}
}
