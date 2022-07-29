use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::{
    localization, CurrentLocale, LocalizationBundle, LocalizationError, LocalizationPlugin,
    LocalizationSource,
};
use unic_langid::{langid, LanguageIdentifier};

#[derive(LocalizationFolder)]
#[localization(folder = "locale")]
struct ExampleLocalization;

fn main() {
    App::new()
        .insert_resource(Locale(EN::US))
        .add_plugins(DefaultPlugins)
        .add_plugin(LocalizationPlugin)
        .add_localization::<ExampleLocalization>()
        .add_system(use_localization_system)
        // -- snip --
        .run();
}

fn use_localization_system(localization: Option<Res<Locale<ExampleLocalization>>>) {
    if let Some(localization) = localization {
        let msg = localization.try_get_message("hello").unwrap();
        println!("{msg}");
    }
}

fn change_locale_system(locale: ResMut<Locale>) {
    locale.set(DE::DE);
}
