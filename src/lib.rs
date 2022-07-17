use fluent::FluentBundle;

use bevy::{prelude::*, reflect::TypeUuid};
use fluent::FluentResource;
use loaders::ftl_loader::FtlLoader;
use unic_langid::LanguageIdentifier;

mod loaders;

#[derive(PartialEq, Eq, Debug, Clone)]
/// The currently active locale
pub struct CurrentLocale(LanguageIdentifier);

impl CurrentLocale {
    pub fn new(language_id: LanguageIdentifier) -> Self {
        CurrentLocale(language_id)
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

// TODO: Make a fancy derive macro for this
pub trait LocalizationBundle {
    /// Try to get the localization resource handle for the given language ID.
    fn try_get_resource_handle(
        &self,
        language_id: &LanguageIdentifier,
    ) -> Result<Handle<LocalizationSource>, LocalizationError>;

    fn try_get_message(
        &self,
        current_locale: &CurrentLocale,
        assets: Res<Assets<LocalizationSource>>,
        message_id: &str,
    ) -> Result<String, LocalizationError> {
        let language_id = &current_locale.0;
        let handle = self.try_get_resource_handle(language_id)?;

        // Build resource
        let resource = if let Some(source) = assets.get(&handle) {
            Ok(&source.resource)
        } else {
            Err(LocalizationError)
        }?;

        // Build bundle
        let mut bundle = FluentBundle::new(vec![language_id.clone()]);

        if bundle.add_resource(resource).is_err() {
            return Err(LocalizationError);
        }

        // Get message
        let msg = bundle
            .get_message(message_id)
            .expect("Message doesn't exist.");

        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(pattern, None, &mut errors);

        if !errors.is_empty() {
            Err(LocalizationError)
        } else {
            Ok(value.to_string())
        }
    }
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
