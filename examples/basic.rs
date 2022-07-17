use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::{LocalizationPlugin, LocalizationSource};
use fluent::{FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

/// The currently active locale
struct CurrentLocale(LanguageIdentifier);

#[derive(AssetCollection)]
struct ExampleLocalization {
    #[asset(path = "basic/en_us.ftl")]
    en_us: Handle<LocalizationSource>,
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
    localizations: Res<Assets<LocalizationSource>>,
) {
    // TODO: Make this based on the current locale
    if let Some(source) = localizations.get(&handle.en_us) {
        let res: &FluentResource = &source.resource;

        // TODO: The bundle should be created automatically
        let mut bundle = FluentBundle::new(vec![current_locale.0.clone()]);
        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        let msg = bundle.get_message("hello").expect("Message doesn't exist.");

        // TODO: Make this suck less
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(&pattern, None, &mut errors);

        // Print the localized text!
        println!("{}", value);
    } else {
        println!("Localization not loaded yet!");
    }
}
