use bevy::prelude::*;
use bevy_mod_localization::plugin::LocalizationPlugin;

fn main() {
    App::new()
        .add_plugins((LocalizationPlugin, DefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("localized://flag.png"),
        ..default()
    });
}
