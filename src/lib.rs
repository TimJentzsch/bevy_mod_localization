use bevy::{prelude::*, reflect::TypeUuid};
use unic_langid::LanguageIdentifier;

mod fluent;
mod loaders;
pub mod localization;
pub mod plugin;

/// The currently active locale
#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub ftl_string: String,
}

impl LocalizationSource {
    pub fn new(ftl_string: String) -> Self {
        Self { ftl_string }
    }
}

// TODO: Make this a proper error
#[derive(PartialEq, Eq, Debug)]
pub struct LocalizationError;

/// Idk what I'm doing
pub struct LocalizationOutput;

impl FromWorld for LocalizationOutput {
    fn from_world(_world: &mut World) -> Self {
        // ???
        Self
    }
}
