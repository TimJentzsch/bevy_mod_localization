use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_prototype_fluent::{
    localization::AddLocalization,
    localization::{Localization, LocalizationFolder},
    plugin::LocalizationPlugin,
    CurrentLocale,
};
use unic_langid::{langid, LanguageIdentifier};

const EN_US: LanguageIdentifier = langid!("en-US");
const DE: LanguageIdentifier = langid!("de");
const FR: LanguageIdentifier = langid!("fr");

fn main() {
    App::new()
        // Optional: Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LocalizationPlugin)
        .insert_resource(CurrentLocale::new(EN_US))
        // Add the localization resource for the given folder
        .add_localization::<InteractiveLocalizationFolder>()
        .add_startup_system(setup)
        .add_system(text_update_system)
        .add_system(locale_button_system)
        .run();
}

struct InteractiveLocalizationFolder;

// TODO: Write a derive macro for this
impl LocalizationFolder for InteractiveLocalizationFolder {
    fn folder_path() -> String {
        "strings/interactive".to_string()
    }
}

// A unit struct to help identify the localized text component, since there may be many Text components
#[derive(Component)]
struct LocalizedText;

#[derive(Component)]
struct EnglishButton;

#[derive(Component)]
struct GermanButton;

#[derive(Component)]
struct FrenchButton;

/// Update the displayed text, based on the localization files.
fn text_update_system(
    localization: Res<Localization<InteractiveLocalizationFolder>>,
    mut query: Query<&mut Text, With<LocalizedText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok(msg) = localization.try_get_message("hello") {
            // Update the text with the localization
            text.sections[0].value = msg;
        }
    }
}

/// Update the locale when the buttons are clicked.
fn locale_button_system(
    mut current_locale: ResMut<CurrentLocale>,
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&EnglishButton>,
            Option<&GermanButton>,
            Option<&FrenchButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, english, german, french) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Determine the language based on which button got pressed
                let language_id = if let Some(_) = english {
                    EN_US
                } else if let Some(_) = german {
                    DE
                } else if let Some(_) = french {
                    FR
                } else {
                    continue;
                };

                current_locale.update(language_id);
            }
            _ => (),
        }
    }
}

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

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "English",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(EnglishButton);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Deutsch",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(GermanButton);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Français",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(FrenchButton);
}
