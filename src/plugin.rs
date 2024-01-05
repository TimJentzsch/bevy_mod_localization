use bevy::prelude::*;

use crate::{
    asset::plugin::LocalizationAssetPlugin, core::plugin::LocalizationCorePlugin,
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
        app.add_plugins((LocalizationCorePlugin, LocalizationAssetPlugin));

        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>();
    }
}
