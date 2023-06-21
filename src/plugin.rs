use bevy::prelude::*;

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

        // Handle the localization changes after the asset events are generated
        // For some reason, `.after(AssetSet::AssetEvents)` doesn't work
        app.configure_set(LocalizationSet::HandleChanges.in_base_set(CoreSet::Last));
    }
}
