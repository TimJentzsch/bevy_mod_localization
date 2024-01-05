use bevy::{
    app::{App, Plugin},
    asset::AssetApp as _,
};

use super::loader::FtlLoader;

#[derive(Default)]
pub struct LocalizationAssetPlugin;

impl Plugin for LocalizationAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<FtlLoader>();
    }
}
