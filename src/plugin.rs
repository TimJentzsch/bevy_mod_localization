use bevy::prelude::*;

use crate::{
    asset::loader::FtlLoader,
    core::{
        default_fallback::LocaleDefaultFallback, fallback_map::LocaleFallbackMap,
        locale_chain::LocaleChain,
    },
    LocalizationOutput,
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
        app.init_resource::<LocaleChain>();

        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>();

        app.init_asset_loader::<FtlLoader>();
    }
}
