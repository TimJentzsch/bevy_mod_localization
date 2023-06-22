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
use bevy::{asset::AssetPlugin, prelude::*};
use bevy_mod_localization::prelude::*;

#[derive(LocalizationFolder)]
#[folder_path = "strings/locale_fallback"]
struct FallbackLocalizationFolder;

fn main() {
    let mut fallback_map = LocaleFallbackMap::new();
    fallback_map.add_fallback("en-US", "en-GB");

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Optional: Enable hot reloading
            watch_for_changes: true,
            ..default()
        }))
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
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
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
                    .spawn(TextBundle {
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
