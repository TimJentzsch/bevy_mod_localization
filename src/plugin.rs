use bevy::{asset::AssetStage, prelude::*};

use crate::{loaders::ftl_loader::FtlLoader, LocalizationOutput, LocalizationSource};

/// The names of localization stages in an App Schedule
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum LocalizationStage {
    UpdateLocalization,
}

#[derive(Default)]
pub struct LocalizationPlugin {}

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>()
            .add_asset::<LocalizationSource>();

        app.init_asset_loader::<FtlLoader>();

        app.add_stage_after(
            AssetStage::AssetEvents,
            LocalizationStage::UpdateLocalization,
            SystemStage::parallel(),
        );
    }
}
