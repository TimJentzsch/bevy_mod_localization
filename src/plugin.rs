use bevy::{asset::io::AssetSource, prelude::*};

use crate::asset::reader::LocalizedReader;

#[derive(Default)]
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        let asset_source = AssetSource::build()
            // TODO: Get root path from default asset source
            // TODO: Get default locale
            .with_reader(move || {
                Box::new(LocalizedReader::new("assets", "en-US".parse().unwrap()))
            });

        app.register_asset_source("localized", asset_source);
    }
}
