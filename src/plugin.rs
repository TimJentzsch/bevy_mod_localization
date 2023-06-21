use bevy::{asset::AssetSet, prelude::*};

use crate::{
    loaders::ftl_loader::FtlLoader,
    locale::{LocaleDefaultFallback, LocaleFallbackMap},
    LocalizationOutput, LocalizationSource,
};

/// The sets containing the localization systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum LocalizationSet {
    HandleChanges,
}

#[derive(Default)]
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocaleFallbackMap>();
        app.init_resource::<LocaleDefaultFallback>();

        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>()
            .add_asset::<LocalizationSource>();

        app.init_asset_loader::<FtlLoader>();

        app.configure_set(LocalizationSet::HandleChanges.after(AssetSet::AssetEvents));
    }
}
