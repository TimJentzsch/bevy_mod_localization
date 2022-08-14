use bevy::prelude::*;
use fluent::FluentResource;
use unic_langid::LanguageIdentifier;

use super::{
    utils::{get_ftl_path, get_resolution_chain},
    Localization, LocalizationFolder,
};
use crate::{
    fluent::FluentBundle,
    locale::{Locale, LocaleDefaultFallback, LocaleFallbackMap},
    LocalizationSource,
};

/// Load the corresponding localization file when the locale has been changed.
pub fn update_localization_on_locale_change<T: LocalizationFolder>(
    mut localization: ResMut<Localization<T>>,
    asset_server: ResMut<AssetServer>,
    locale: Res<Locale>,
    fallback_map: Res<LocaleFallbackMap>,
    default_fallback: Res<LocaleDefaultFallback>,
) {
    if locale.is_changed() || fallback_map.is_changed() || default_fallback.is_changed() {
        // Compute the new resolution chain
        let resolution_chain = get_resolution_chain(&locale, &fallback_map, &default_fallback);

        let handle_keys: Vec<LanguageIdentifier> =
            localization.handle_map.keys().cloned().collect();

        // Load handles that are now needed
        for lang_id in resolution_chain
            .iter()
            .filter(|lang_id| !handle_keys.contains(lang_id))
        {
            let ftl_path = get_ftl_path::<T>(lang_id);
            let handle = asset_server.load(ftl_path);
            localization.handle_map.insert(lang_id.clone(), handle);
        }

        // Remove handles that are no longer needed
        for lang_id in handle_keys
            .iter()
            .filter(|lang_id| !resolution_chain.contains(lang_id))
        {
            localization.handle_map.remove(lang_id);
            localization.bundle_map.remove(lang_id);
        }

        // Update resolution chain
        localization.resolution_chain = resolution_chain;
    }
}

/// Update the localization resource when a localization asset has been changed.
///
/// This happens in the following scenarios:
/// - The localization file has been loaded for the first time.
/// - The localization file has been edited and hot-reloading is enabled.
/// - The locale has been changed, so a new localization file has been loaded.
pub fn update_localization_on_asset_change<T: LocalizationFolder>(
    mut localization: ResMut<Localization<T>>,
    mut ev_asset: EventReader<AssetEvent<LocalizationSource>>,
    assets: ResMut<Assets<LocalizationSource>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                let lang_id = localization.handle_map.iter().find_map(|(key, value)| {
                    if *value == *handle {
                        Some(key.clone())
                    } else {
                        None
                    }
                });

                if let Some(lang_id) = lang_id {
                    let source = assets
                        .get(handle)
                        .expect("Localization source expected to be loaded but it wasn't!");

                    // TODO: Make this more efficient, the parsing could take some time
                    let resource = FluentResource::try_new(source.ftl_string.clone())
                        .expect("Failed to parse an FTL string.");

                    let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
                    bundle
                        .add_resource(resource)
                        .expect("Failed to add resource to bundle");

                    localization.bundle_map.insert(lang_id.clone(), bundle);
                }
            }
            AssetEvent::Removed { handle } => {
                let lang_id = localization.handle_map.iter().find_map(|(key, value)| {
                    if *value == *handle {
                        Some(key.clone())
                    } else {
                        None
                    }
                });

                if let Some(lang_id) = lang_id {
                    localization.bundle_map.remove(&lang_id);
                }
            }
        }
    }
}
