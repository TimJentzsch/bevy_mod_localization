use bevy::{prelude::*, reflect::TypeUuid};

pub mod asset;
pub mod core;
pub mod plugin;

#[derive(Debug, TypeUuid, Asset, TypePath)]
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
