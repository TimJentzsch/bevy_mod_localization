use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{fluent::FluentBundle, plugin::LocalizationStage, CurrentLocale, LocalizationSource};
use bevy::prelude::*;
use fluent::FluentResource;

use crate::LocalizationError;

// TODO: Review if the 'static is really needed for world.contains_resource
pub trait LocalizationFolder: 'static + std::marker::Send + std::marker::Sync {
    fn folder_path() -> String;
}

pub struct Localization<T: LocalizationFolder> {
    phantom: std::marker::PhantomData<T>,
    handle: Handle<LocalizationSource>,
    cur_bundle: Option<FluentBundle>,
}

impl<T: LocalizationFolder> Localization<T> {
    pub fn new(handle: Handle<LocalizationSource>) -> Self {
        Self {
            phantom: PhantomData,
            handle,
            cur_bundle: None,
        }
    }

    pub fn try_get_message(&self, message_id: &str) -> Result<String, LocalizationError> {
        let bundle = if let Some(bundle) = &self.cur_bundle {
            bundle
        } else {
            return Err(LocalizationError);
        };

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

pub trait AddLocalization {
    fn add_localization<T: LocalizationFolder>(&mut self) -> &mut Self;
}

impl AddLocalization for App {
    fn add_localization<T: LocalizationFolder>(&mut self) -> &mut Self {
        if self.world.contains_resource::<Localization<T>>() {
            return self;
        }

        let locale = self.world.resource::<CurrentLocale>();
        let asset_server = self.world.resource::<AssetServer>();

        let ftl_path = get_ftl_path::<T>(locale);
        let handle: Handle<LocalizationSource> = asset_server.load(ftl_path);

        let localization = Localization::<T>::new(handle);

        self.insert_resource(localization)
            // First, check if the locale changed
            .add_system_to_stage(
                LocalizationStage::HandleChanges,
                update_localization_on_locale_change::<T>,
            )
            // Then check if the asset changed
            // A locale change will also reload the assets, so this has to happen afterwards
            .add_system_to_stage(
                LocalizationStage::HandleChanges,
                update_localization_on_asset_change::<T>
                    .after(update_localization_on_locale_change::<T>),
            );

        self
    }
}

/// Get the path of the FTL file for the given locale in the localization folder.
fn get_ftl_path<T: LocalizationFolder>(locale: &CurrentLocale) -> PathBuf {
    Path::new(&T::folder_path()).join(format!("{}.ftl", locale.0))
}

/// Load the corresponding localization file when the locale has been changed.
fn update_localization_on_locale_change<T: LocalizationFolder>(
    mut localization: ResMut<Localization<T>>,
    asset_server: ResMut<AssetServer>,
    cur_locale: Res<CurrentLocale>,
) {
    if cur_locale.is_changed() {
        let ftl_path = get_ftl_path::<T>(&cur_locale);
        let handle: Handle<LocalizationSource> = asset_server.load(ftl_path);

        localization.handle = handle;
    }
}

/// Update the localization resource when a localization asset has been changed.
///
/// This happens in the following scenarios:
/// - The localization file has been loaded for the first time.
/// - The localization file has been edited and hot-reloading is enabled.
/// - The locale has been changed, so a new localization file has been loaded.
fn update_localization_on_asset_change<T: LocalizationFolder>(
    mut localization: ResMut<Localization<T>>,
    mut ev_asset: EventReader<AssetEvent<LocalizationSource>>,
    assets: ResMut<Assets<LocalizationSource>>,
    cur_locale: Res<CurrentLocale>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if *handle != localization.handle {
                    continue;
                }

                let source = assets
                    .get(handle)
                    .expect("Localization source expected to be loaded but it wasn't!");

                // TODO: Make this more efficient, the parsing could take some time
                let resource = FluentResource::try_new(source.ftl_string.clone())
                    .expect("Failed to parse an FTL string.");

                let lang_id = cur_locale.0.clone();

                let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
                bundle
                    .add_resource(resource)
                    .expect("Failed to add resource to bundle");

                localization.cur_bundle = Some(bundle);
            }
            AssetEvent::Removed { handle } => {
                if *handle != localization.handle {
                    continue;
                }

                localization.cur_bundle = None;
            }
        }
    }
}
