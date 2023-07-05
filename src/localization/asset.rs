use std::marker::PhantomData;

use bevy::{asset::Asset, prelude::*, utils::HashMap};
use unic_langid::LanguageIdentifier;

use super::resolution_chain::LocaleResolutionChain;

/// Obtain a handle to an asset based on the current [`Locale`](crate::locale::Locale).
///
/// The component will be updated automatically when the locale changes,
/// make sure that you update the handle where you are using it when the change event is triggered.
#[derive(Component)]
pub struct LocalizedAsset<A: Asset> {
    /// The path to the folder where the localized asset is located.
    folder_path: &'static str,

    /// The extension of the asset files.
    extension: &'static str,

    /// A map from the locale IDs to the corresponding asset handles.
    handle_map: HashMap<LanguageIdentifier, Handle<A>>,
}

impl<A: Asset> LocalizedAsset<A> {
    /// Create a new localized asset from the given folder.
    ///
    /// The folder should contain files named after the locale identifiers.
    /// The files should all have the specified extension.
    pub fn new(folder_path: &'static str, extension: &'static str) -> Self {
        LocalizedAsset {
            folder_path,
            extension,
            handle_map: HashMap::new(),
        }
    }

    /// Get the path to the asset of the given locale.
    pub fn get_asset_path(&self, locale_id: LanguageIdentifier) -> String {
        format!(
            "{}/{}.{}",
            self.folder_path,
            locale_id.into(),
            self.extension
        )
    }
}

pub fn update_asset_on_locale_changes<A: Asset>(
    mut query: Query<&mut LocalizedAsset<A>>,
    resolution_chain: Res<LocaleResolutionChain>,
    mut asset_server: ResMut<AssetServer>,
) {
    if !resolution_chain.is_changed() {
        return;
    }

    for mut localized_asset in query.iter_mut() {
        let loaded_locales = localized_asset.handle_map.keys().clone();

        // Unload assets that are not needed anymore
        // TODO: Only unload them after the new assets have been loaded
        for locale_id in loaded_locales {
            if !resolution_chain.chain.contains(locale_id) {
                localized_asset.handle_map.remove(locale_id);
            }
        }

        // Load newly needed assets
        // TODO: Lazily load them in the order of the resolution chain
        for locale_id in resolution_chain.chain {
            if localized_asset.handle_map.contains_key(&locale_id) {
                continue;
            }

            let handle = asset_server.load(localized_asset.get_asset_path(locale_id));
            localized_asset.handle_map.insert(locale_id, handle);
        }
    }
}

/// A plugin to manage the localization of the asset of the given generic type.
struct LocalizedAssetPlugin<A: Asset> {
    /// Make the compiler happy by using the generic parameter.
    _asset: PhantomData<A>,
}

impl<A: Asset> Plugin for LocalizedAssetPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_systems(update_asset_on_locale_changes::<A>);
    }
}
