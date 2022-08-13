use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{
    fluent::FluentBundle,
    locale::{Locale, LocaleDefaultFallback, LocaleFallbackMap},
    plugin::LocalizationStage,
    LocalizationSource,
};
use bevy::{prelude::*, utils::HashMap};
use fluent::{FluentArgs, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::LocalizationError;

/// A folder containing localization files.
///
/// This trait is used to define the path to a localization folder.
/// The path works like asset paths, i.e. by default, `asset` is the root of the path.
/// Instead of implementing it manually, it is recommended to use the derive macro:
///
/// ```
/// # use bevy_prototype_fluent::prelude::*;
/// #
/// #[derive(LocalizationFolder)]
/// #[folder_path = "/strings/example"]
/// struct ExampleLocalizationFolder;
///
/// assert_eq!(ExampleLocalizationFolder::FOLDER_PATH, "/strings/example");
/// ```
///
/// By default, this will point to the folder `/assets/strings/example` in your crate.
///
/// The folder should then contain `.ftl` files for each language you want to support.
/// The files must be named after the corresponding unicode language tag they represent.
/// The following structure corresponds to the example above:
///
/// ```txt
/// my_crate/
/// ├─ assets/
/// │  ├─ strings/
/// │  │  ├─ example/
/// │  │  │  ├─ en-US.ftl
/// │  │  │  ├─ de.ftl
/// │  │  │  ├─ fr.ftl
/// ```
// TODO: Review if the 'static is really needed for world.contains_resource
pub trait LocalizationFolder: 'static + std::marker::Send + std::marker::Sync {
    const FOLDER_PATH: &'static str;
}

pub struct Localization<T: LocalizationFolder> {
    phantom: std::marker::PhantomData<T>,
    pub(crate) handle_map: HashMap<LanguageIdentifier, Handle<LocalizationSource>>,
    pub(crate) bundle_map: HashMap<LanguageIdentifier, FluentBundle>,
    pub(crate) resolution_chain: Vec<LanguageIdentifier>,
}

impl<T: LocalizationFolder> Localization<T> {
    pub fn new(resolution_chain: Vec<LanguageIdentifier>) -> Self {
        Self {
            resolution_chain,
            ..Default::default()
        }
    }

    fn try_format_pattern(
        &self,
        message_id: &str,
        args: Option<&FluentArgs>,
    ) -> Result<String, LocalizationError> {
        for lang_id in &self.resolution_chain {
            if let Some(bundle) = self.bundle_map.get(lang_id) {
                if let Some(msg) = bundle.get_message(message_id) {
                    let mut errors = vec![];

                    if let Some(pattern) = msg.value() {
                        let formatted_message = bundle.format_pattern(pattern, args, &mut errors);

                        if errors.is_empty() {
                            return Ok(formatted_message.to_string());
                        }
                    }
                }
            }
        }

        Err(LocalizationError)
    }

    pub fn try_get_message(&self, message_id: &str) -> Result<String, LocalizationError> {
        self.try_format_pattern(message_id, None)
    }

    pub fn try_format_message(
        &self,
        message_id: &str,
        args: FluentArgs,
    ) -> Result<String, LocalizationError> {
        self.try_format_pattern(message_id, Some(&args))
            // The inserted values are wrapped in U+2058 (First Strong Isolate) and U+2069 (Pop Directional Isolate)
            // The font can't handle them, so we replace them for now
            // TODO: Don't do this
            .map(|msg| msg.replace('\u{2068}', "").replace('\u{2069}', ""))
    }
}

impl<T: LocalizationFolder> Default for Localization<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            resolution_chain: Vec::new(),
            handle_map: HashMap::default(),
            bundle_map: HashMap::default(),
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

        let asset_server = self.world.resource::<AssetServer>();

        let locale = self.world.resource::<Locale>();
        let fallback_map = self.world.resource::<LocaleFallbackMap>();
        let default_fallback = self.world.resource::<LocaleDefaultFallback>();

        let resolution_chain = get_resolution_chain(locale, fallback_map, default_fallback);

        let mut localization = Localization::<T>::new(resolution_chain.clone());

        // Initiate loading of the localization files
        for lang_id in resolution_chain {
            let ftl_path = get_ftl_path::<T>(&lang_id);
            let handle: Handle<LocalizationSource> = asset_server.load(ftl_path);
            localization.handle_map.insert(lang_id.clone(), handle);
        }

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

/// Get the path of the FTL file for the given language ID in the localization folder.
fn get_ftl_path<T: LocalizationFolder>(lang_id: &LanguageIdentifier) -> PathBuf {
    Path::new(&T::FOLDER_PATH).join(format!("{}.ftl", lang_id))
}

fn get_resolution_chain(
    locale: &Locale,
    fallback_map: &LocaleFallbackMap,
    default_fallback: &LocaleDefaultFallback,
) -> Vec<LanguageIdentifier> {
    let locale = locale.0.clone();
    let fallbacks = fallback_map.0.get(&locale);
    let default_fallback = default_fallback.0.clone();

    let mut resolution_chain = Vec::with_capacity(fallbacks.map_or(0, |f| f.len()) + 2);

    resolution_chain.push(locale);

    if let Some(fallbacks) = fallbacks {
        resolution_chain.append(&mut fallbacks.clone());
    }

    if let Some(default_fallback) = default_fallback {
        resolution_chain.push(default_fallback);
    }

    resolution_chain
}

/// Load the corresponding localization file when the locale has been changed.
fn update_localization_on_locale_change<T: LocalizationFolder>(
    mut localization: ResMut<Localization<T>>,
    asset_server: ResMut<AssetServer>,
    locale: Res<Locale>,
    fallback_map: Res<LocaleFallbackMap>,
    default_fallback: Res<LocaleDefaultFallback>,
) {
    if locale.is_changed() || fallback_map.is_changed() || default_fallback.is_changed() {
        // Compute the new resolution chain
        let resolution_chain = get_resolution_chain(&locale, &fallback_map, &default_fallback);

        let handle_keys: Vec<LanguageIdentifier> =
            localization.handle_map.keys().cloned().collect();

        // Load handles that are now needed
        for lang_id in resolution_chain
            .iter()
            .filter(|lang_id| !handle_keys.contains(lang_id))
        {
            let ftl_path = get_ftl_path::<T>(lang_id);
            let handle = asset_server.load(ftl_path);
            localization.handle_map.insert(lang_id.clone(), handle);
        }

        // Remove handles that are no longer needed
        for lang_id in handle_keys
            .iter()
            .filter(|lang_id| !resolution_chain.contains(lang_id))
        {
            localization.handle_map.remove(lang_id);
            localization.bundle_map.remove(lang_id);
        }

        // Update resolution chain
        localization.resolution_chain = resolution_chain;
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
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                let lang_id = localization.handle_map.iter().find_map(|(key, value)| {
                    if *value == *handle {
                        Some(key.clone())
                    } else {
                        None
                    }
                });

                if let Some(lang_id) = lang_id {
                    let source = assets
                        .get(handle)
                        .expect("Localization source expected to be loaded but it wasn't!");

                    // TODO: Make this more efficient, the parsing could take some time
                    let resource = FluentResource::try_new(source.ftl_string.clone())
                        .expect("Failed to parse an FTL string.");

                    let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
                    bundle
                        .add_resource(resource)
                        .expect("Failed to add resource to bundle");

                    localization.bundle_map.insert(lang_id.clone(), bundle);
                }
            }
            AssetEvent::Removed { handle } => {
                let lang_id = localization.handle_map.iter().find_map(|(key, value)| {
                    if *value == *handle {
                        Some(key.clone())
                    } else {
                        None
                    }
                });

                if let Some(lang_id) = lang_id {
                    localization.bundle_map.remove(&lang_id);
                }
            }
        }
    }
}
