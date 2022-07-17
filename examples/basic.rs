use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::{
    LocalizationBundle, LocalizationError, LocalizationPlugin, LocalizationSource,
};
use fluent::{FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

/// The currently active locale
struct CurrentLocale(LanguageIdentifier);

#[derive(AssetCollection)]
struct ExampleLocalization {
    #[asset(path = "basic/en_us.ftl")]
    en_us: Handle<LocalizationSource>,
}

// TODO: This should be implemented with a fancy derive macro instead
impl LocalizationBundle for ExampleLocalization {
    fn try_get_resource_handle(
        &self,
        language_id: &LanguageIdentifier,
    ) -> Result<Handle<LocalizationSource>, LocalizationError> {
        if *language_id == langid!("en-US") {
            Ok(self.en_us.clone())
        } else {
            Err(LocalizationError)
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LocalizationPlugin)
        .insert_resource(CurrentLocale(langid!("en-US")))
        .init_collection::<ExampleLocalization>()
        .add_system(print_text)
        .run();
}

fn print_text(
    current_locale: Res<CurrentLocale>,
    handle: Res<ExampleLocalization>,
    assets: Res<Assets<LocalizationSource>>,
) {
    if let Ok(msg) = handle.try_get_message(&current_locale.0, assets, "hello") {
        println!("{msg}");
    }
}
