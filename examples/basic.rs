use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::LocalizationSource;
use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;

#[derive(AssetCollection)]
struct ExampleLocalization {
    #[asset(path = "basic/en_us.ftl")]
    en_us: Handle<LocalizationSource>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_collection::<ExampleLocalization>()
        .add_startup_system(print_text)
        .run();
}

fn print_text(handle: Res<ExampleLocalization>, localizations: Res<Assets<LocalizationSource>>) {
    // TODO: Make this based on the current locale
    if let Some(source) = localizations.get(&handle.en_us) {
        let res: &FluentResource = &source.resource;

        // TODO: The bundle should be created automatically
        let langid_en = langid!("en-US");
        let mut bundle = FluentBundle::new(vec![langid_en]);

        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        let msg = bundle
            .get_message("hello-world")
            .expect("Message doesn't exist.");

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
