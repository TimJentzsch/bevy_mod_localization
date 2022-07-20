use crate::fluent::FluentResource;
use bevy::{prelude::*, reflect::TypeUuid};
use loaders::ftl_loader::FtlLoader;
use unic_langid::LanguageIdentifier;

mod fluent;
mod loaders;
pub mod localization;

#[derive(PartialEq, Eq, Debug, Clone)]
/// The currently active locale
pub struct CurrentLocale(LanguageIdentifier);

impl CurrentLocale {
    pub fn new(language_id: LanguageIdentifier) -> Self {
        CurrentLocale(language_id)
    }

    pub fn update(&mut self, language_id: LanguageIdentifier) {
        self.0 = language_id;
    }
}

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

// TODO: Make this a proper error
#[derive(PartialEq, Eq)]
pub struct LocalizationError;

/// Idk what I'm doing
pub struct LocalizationOutput;

impl FromWorld for LocalizationOutput {
    fn from_world(_world: &mut World) -> Self {
        // ???
        Self
    }
}

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
