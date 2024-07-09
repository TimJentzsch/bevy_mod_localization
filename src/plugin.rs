use bevy::{asset::io::AssetSource, prelude::*};

use crate::{
    asset::reader::LocalizedReader,
    locale::{Locale, LocaleId},
};

#[derive(Default)]
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Allow user to set default locale
        let locale_id: LocaleId = "en-US".parse().unwrap();
        let locale = Locale {
            locale_id: locale_id.clone(),
        };

        let asset_source = AssetSource::build()
            // TODO: Get root path from default asset source
            .with_reader(move || Box::new(LocalizedReader::new("assets", locale_id.clone())));

        app.register_asset_source("localized", asset_source)
            .insert_resource(locale);
    }
}
