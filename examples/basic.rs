use bevy::{
    asset::{AssetPlugin, AssetServerSettings},
    prelude::*,
};
use bevy_prototype_fluent::{
    localization::AddLocalization,
    localization::{Localization, LocalizationFolder},
    plugin::LocalizationPlugin,
    CurrentLocale,
};
use unic_langid::langid;

struct ExampleLocalization;

// TODO: Write a derive macro for this
impl LocalizationFolder for ExampleLocalization {
    fn folder_path() -> String {
        "strings/basic".to_string()
    }
}

fn main() {
    App::new()
        // Optional: Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        // Define the locale that you want to use by default
        .insert_resource(CurrentLocale::new(langid!("en-US")))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(LocalizationPlugin)
        // Add the localization resource for the given folder
        .add_localization::<ExampleLocalization>()
        // Do something with it!
        .add_system(use_localization_system)
        .run();
}

fn use_localization_system(localization: Res<Localization<ExampleLocalization>>) {
    if let Ok(msg) = localization.try_get_message("hello") {
        println!("{msg}");
    }
}
