use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{AppState, GameAssets};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Main), spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands, scene_assets: Res<GameAssets>) {
    commands.spawn((
        SceneBundle {
            scene: scene_assets.room.clone(),
            ..default()
        },
        RigidBody::Static,
        AsyncSceneCollider::new(Some(ComputedCollider::ConvexDecomposition(
            VHACDParameters::default(),
        ))),
    ));
}
