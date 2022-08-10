use bevy::{
    asset::{AssetPlugin, AssetServerSettings},
    prelude::*,
};
use bevy_prototype_fluent::prelude::*;
use unic_langid::langid;

fn main() {
    App::new()
        // Optional: Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        // Define the locale that you want to use by default
        .insert_resource(Locale::new(langid!("en-US")))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(LocalizationPlugin)
        // Add the localization resource for the given folder
        .add_localization::<BasicLocalizationFolder>()
        // Do something with it!
        .add_system(print_message_system)
        .run();
}

struct BasicLocalizationFolder;

// TODO: Write a derive macro for this
impl LocalizationFolder for BasicLocalizationFolder {
    fn folder_path() -> String {
        "strings/basic".to_string()
    }
}

/// Print the text from the localization file.
fn print_message_system(localization: Res<Localization<BasicLocalizationFolder>>) {
    if let Ok(msg) = localization.try_get_message("hello") {
        println!("{msg}");
    }
}
