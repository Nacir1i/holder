use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub character: Handle<Scene>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, assets_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        character: assets_server.load("Guy.glb#Scene0"),
    }
}
