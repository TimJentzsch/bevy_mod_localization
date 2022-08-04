use bevy::{asset::AssetPlugin, prelude::*};
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
        .insert_resource(CurrentLocale::new(langid!("en-US")))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(LocalizationPlugin)
        .add_localization::<ExampleLocalization>()
        .add_system(use_localization_system)
        .run();
}

fn use_localization_system(localization: Res<Localization<ExampleLocalization>>) {
    if let Ok(msg) = localization.try_get_message("hello") {
        println!("{msg}");
    }
}
