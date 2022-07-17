use fluent::FluentBundle;

use bevy::{prelude::*, reflect::TypeUuid};
use fluent::FluentResource;
use loaders::ftl_loader::FtlLoader;

mod loaders;

#[derive(Debug, TypeUuid)]
#[uuid = "c807fa98-31ad-4d85-8988-ab4313cced3f"]
pub struct LocalizationSource {
    pub resource: FluentResource,
}

impl LocalizationSource {
    pub fn new(resource: FluentResource) -> Self {
        Self { resource }
    }
}

pub trait LocalizationBundle<R> {
    fn bundle(&self) -> FluentBundle<R>;
}

/// Idk what I'm doing
pub struct LocalizationOutput;

impl FromWorld for LocalizationOutput {
    fn from_world(_world: &mut World) -> Self {
        // ???
        Self
    }
}

#[derive(Default)]
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // What does this do??
        app.init_non_send_resource::<LocalizationOutput>()
            .add_asset::<LocalizationSource>();

        app.init_asset_loader::<FtlLoader>();
    }
}
