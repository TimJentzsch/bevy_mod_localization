use bevy::prelude::*;

use crate::{loaders::ftl_loader::FtlLoader, LocalizationOutput, LocalizationSource};

#[derive(Default)]
pub struct LocalizationPlugin {}

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>()
            .add_asset::<LocalizationSource>();

        app.init_asset_loader::<FtlLoader>();
    }
}
