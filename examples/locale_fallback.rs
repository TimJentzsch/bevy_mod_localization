//! Example showcasing how to define fallback locales.
//!
//! The general pipeline is as follows:
//!
//! [`Locale`] -> [`LocaleFallbackMap`] -> [`LocaleDefaultFallback`]
//!
//! That means, the message will first be retrieved for the currently active [`Locale`].
//! If the message (or the localization file for this locale) does not exist,
//! the locale will be looked up in the [`LocaleFallbackMap`] to see if any fallbacks are defined.
//! If not or if the message or files don't exist for those locales too,
//! the [`LocaleDefaultFallback`] will be used.
//!
//! This example uses the following configuration:
//!
//! - [`Locale`]: `en-US`
//! - [`LocaleFallbackMap`]: { `en-US` -> `en-GB` }
//! - [`LocaleDefaultFallback`]: `de`
//!
//! The functionality is demonstrated with three messages:
//!
//! - Message 1: Defined for `en-US`, `en-GB` and `de`.
//! - Message 2: Defined for `en-GB` and `de`.
//! - Message 3: Defined for `de`.
//!
//! You can look at `/assets/strings/locale_fallback/` in this repository to see the definitions.
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_prototype_fluent::prelude::*;

#[derive(LocalizationFolder)]
#[folder_path = "strings/locale_fallback"]
struct FallbackLocalizationFolder;

fn main() {
    let mut fallback_map = LocaleFallbackMap::new();
    fallback_map.add_fallback("en-US", "en-GB");

    App::new()
        // Optional: Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // First, try American English
        .insert_resource(Locale::new("en-US"))
        // The fallback for American English is British English
        .insert_resource(fallback_map)
        // The "default" language of this app is German, fallback to that otherwise
        .insert_resource(LocaleDefaultFallback::new(Some("de")))
        .add_plugin(LocalizationPlugin)
        // Add the localization resource for the given folder
        .add_localization::<FallbackLocalizationFolder>()
        .add_startup_system(setup)
        .run();
}

/// Spawn the camera and text nodes.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

            // Create three text messages to demonstrate the behavior
            // - Message 1: Defined for `en-US`, `en-GB` and `de`.
            // - Message 2: Defined for `en-GB` and `de`.
            // - Message 3: Defined for `de`.
            for component in [
                LocalizedText::<FallbackLocalizationFolder>::new("first"),
                LocalizedText::<FallbackLocalizationFolder>::new("second"),
                LocalizedText::<FallbackLocalizationFolder>::new("third"),
            ] {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::from_section(
                            // This will later be replaced by the localized text
                            "",
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    })
                    .insert(component);
            }
        });
}
