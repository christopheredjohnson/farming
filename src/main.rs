mod game;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::game::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TiledMapPlugin::default(),
            GamePlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load a map then spawn it
    commands.spawn((
        // Only the [TiledMapHandle] component is actually required to spawn a map
        TiledMapHandle(asset_server.load("maps/farm.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
        Transform::from_scale(Vec3::splat(2.0))
    ));

    commands.spawn(Camera2d);
}
