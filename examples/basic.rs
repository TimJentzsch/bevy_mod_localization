use bevy::{asset::AssetPlugin, prelude::*};
use bevy_mod_localization::prelude::*;

fn main() {
    App::new()
        // Define the locale that you want to use by default
        .insert_resource(Locale::new("en-US"))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin {
            // Optional: Enable hot reloading
            watch_for_changes: true,
            ..default()
        })
        .add_plugin(LocalizationPlugin)
        // Add the localization resource for the given folder
        .add_localization::<BasicLocalizationFolder>()
        // Do something with it!
        .add_system(print_message_system)
        .run();
}

#[derive(LocalizationFolder)]
#[folder_path = "strings/basic"]
struct BasicLocalizationFolder;

/// Print the text from the localization file.
fn print_message_system(localization: Res<Localization<BasicLocalizationFolder>>) {
    if let Ok(msg) = localization.try_get_message("hello") {
        println!("{msg}");
    }
}
