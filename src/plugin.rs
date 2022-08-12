use bevy::{asset::AssetStage, prelude::*};

use crate::{
    loaders::ftl_loader::FtlLoader,
    locale::{LocaleDefaultFallback, LocaleFallbackMap},
    LocalizationOutput, LocalizationSource,
};

/// The names of localization stages in an App Schedule
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum LocalizationStage {
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

        // Stage to update localization after the file or locale has been loaded/changed
        app.add_stage_after(
            AssetStage::AssetEvents,
            LocalizationStage::HandleChanges,
            SystemStage::parallel(),
        );
    }
}
