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

#[derive(Component)]
struct MessageId(&'static str);

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
        .add_system(update_text_system)
        .run();
}

/// Update the text messages.
///
/// - Message 1: Defined for `en-US`, `en-GB` and `de`.
/// - Message 2: Defined for `en-GB` and `de`.
/// - Message 3: Defined for `de`.
fn update_text_system(
    localization: Res<Localization<FallbackLocalizationFolder>>,
    mut query: Query<(&mut Text, &MessageId)>,
) {
    for (mut text, message_id) in query.iter_mut() {
        if let Ok(msg) = localization.try_get_message(message_id.0) {
            text.sections[0].value = msg;
        }
    }
}

/// Spawn the camera and text nodes.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

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
            for component in [MessageId("first"), MessageId("second"), MessageId("third")] {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            // This will later be replaced by the localized text
                            "",
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..default()
                            },
                        ),
                        ..default()
                    })
                    .insert(component);
            }
        });
}
