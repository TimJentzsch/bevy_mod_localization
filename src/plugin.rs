use std::time::Duration;

use bevy::{asset::io::AssetSource, prelude::*};

use crate::{
    asset::{reader::LocalizedReader, watcher::get_localized_watcher},
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

        // TODO: Get root path from default asset source
        let root_path = "assets";

        let asset_source = AssetSource::build()
            .with_reader(move || Box::new(LocalizedReader::new(root_path, locale_id.clone())))
            .with_watcher(get_localized_watcher(
                root_path.into(),
                Duration::from_millis(300),
            ));

        app.register_asset_source("localized", asset_source)
            .insert_resource(locale);
    }
}
