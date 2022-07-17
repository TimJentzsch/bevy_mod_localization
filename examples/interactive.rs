use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::{
    CurrentLocale, LocalizationBundle, LocalizationError, LocalizationPlugin, LocalizationSource,
};
use unic_langid::{langid, LanguageIdentifier};

#[derive(AssetCollection)]
struct ExampleLocalization {
    #[asset(path = "strings/interactive/en_us.ftl")]
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
        .insert_resource(CurrentLocale::new(langid!("en-US")))
        .init_collection::<ExampleLocalization>()
        .add_startup_system(setup)
        .add_system(text_update_system)
        .run();
}

// A unit struct to help identify the localized text component, since there may be many Text components
#[derive(Component)]
struct LocalizedText;

/// Spawn the camera and text node
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },

            text: Text::with_section(
                // This will later be replaced by the localized text
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(LocalizedText);
}

fn text_update_system(
    current_locale: Res<CurrentLocale>,
    handle: Res<ExampleLocalization>,
    assets: Res<Assets<LocalizationSource>>,
    mut query: Query<&mut Text, With<LocalizedText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok(msg) = handle.try_get_message(&current_locale, assets, "hello") {
            // Update the text with the localization
            text.sections[0].value = msg;
        }
    }
}
