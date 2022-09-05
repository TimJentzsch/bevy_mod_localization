use crate::{
    plugin::LocalizationStage,
    prelude::{Locale, LocaleDefaultFallback, LocaleFallbackMap},
    LocalizationSource,
};

use super::{
    localization_args::LocalizationArgs,
    systems::{
        update_localization_on_asset_change, update_localization_on_locale_change,
        update_localized_text,
    },
    utils::{get_ftl_path, get_resolution_chain},
    Localization, LocalizationFolder,
};
use bevy::prelude::*;

pub trait AddLocalization {
    fn add_localization<F: LocalizationFolder>(&mut self) -> &mut Self;
}

impl AddLocalization for App {
    fn add_localization<F: LocalizationFolder>(&mut self) -> &mut Self {
        if self.world.contains_resource::<Localization<F>>() {
            return self;
        }

        let asset_server = self.world.resource::<AssetServer>();

        let locale = self.world.resource::<Locale>();
        let fallback_map = self.world.resource::<LocaleFallbackMap>();
        let default_fallback = self.world.resource::<LocaleDefaultFallback>();

        let resolution_chain = get_resolution_chain(locale, fallback_map, default_fallback);

        let mut localization = Localization::<F>::new(resolution_chain.clone());

        // Initiate loading of the localization files
        for lang_id in resolution_chain {
            let ftl_path = get_ftl_path::<F>(&lang_id);
            let handle: Handle<LocalizationSource> = asset_server.load(ftl_path);
            localization.handle_map.insert(lang_id.clone(), handle);
        }

        self.insert_resource(localization).add_system_set_to_stage(
            LocalizationStage::HandleChanges,
            SystemSet::new()
                // First, check if the locale changed
                .with_system(update_localization_on_locale_change::<F>)
                // Then check if the asset changed
                // A locale change will also reload the assets, so this has to happen afterwards
                .with_system(
                    update_localization_on_asset_change::<F>
                        .after(update_localization_on_locale_change::<F>),
                )
                // Update localized text components
                .with_system(
                    update_localized_text::<F, LocalizationArgs>
                        .after(update_localization_on_asset_change::<F>),
                ),
        );

        self
    }
}
