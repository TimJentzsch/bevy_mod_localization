use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_prototype_fluent::prelude::*;
use fluent::FluentArgs;

#[derive(LocalizationFolder)]
#[folder_path = "strings/interactive"]
struct InteractiveLocalizationFolder;

/// Tag for a text with a simple message.
#[derive(Component)]
struct WelcomeText;

/// Tag for a text that takes a count as argument.
#[derive(Component)]
struct AppleText;

/// The count for the parameterized text.
struct AppleCount(usize);

#[derive(Component)]
struct CountIncrementButton;

#[derive(Component)]
struct CountDecrementButton;

#[derive(Component)]
struct LanguageButton(&'static str);

fn main() {
    App::new()
        // Optional: Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Locale::new("en-US"))
        .add_plugin(LocalizationPlugin)
        // Add the localization resource for the given folder
        .add_localization::<InteractiveLocalizationFolder>()
        // Initialize the count to 0
        .insert_resource(AppleCount(0))
        .add_startup_system(setup)
        .add_system(simple_text_update_system)
        .add_system(parameterized_text_update_system)
        .add_system(locale_button_system)
        .add_system(count_button_system)
        .run();
}

/// Update the displayed text, based on the localization files.
fn simple_text_update_system(
    localization: Res<Localization<InteractiveLocalizationFolder>>,
    mut query: Query<&mut Text, With<WelcomeText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok(msg) = localization.try_get_message("hello") {
            // Update the text with the localization
            text.sections[0].value = msg;
        }
    }
}

fn parameterized_text_update_system(
    localization: Res<Localization<InteractiveLocalizationFolder>>,
    mut query: Query<&mut Text, With<AppleText>>,
    count: Res<AppleCount>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let mut args = FluentArgs::new();
        args.set("count", count.0);

        if let Ok(msg) = localization.try_format_message("apple-count", args) {
            // Update the text with the localization
            text.sections[0].value = msg;
        }
    }
}

/// Update the locale when the buttons are clicked.
fn locale_button_system(
    mut locale: ResMut<Locale>,
    mut interaction_query: Query<
        (&Interaction, &LanguageButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, language_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Update the locale to the locale of the button
                locale.set(language_button.0);
            }
            _ => (),
        }
    }
}

/// Update the locale when the buttons are clicked.
fn count_button_system(
    mut count: ResMut<AppleCount>,
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&CountIncrementButton>,
            Option<&CountDecrementButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, increment, decrement) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Change the count
                if increment.is_some() {
                    count.0 = count.0.saturating_add(1);
                } else if decrement.is_some() {
                    count.0 = count.0.saturating_sub(1);
                }
            }
            _ => (),
        }
    }
}

/// Spawn the camera and text node
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
            // Node for simple text
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        // This will later be replaced by the localized text
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(WelcomeText);

            // Node for parameterized text
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        // This will later be replaced by the localized text
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(AppleText);

            // Counter
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Counter: ",
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle("+", font.clone()));
                        })
                        .insert(CountIncrementButton);

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle("-", font.clone()));
                        })
                        .insert(CountDecrementButton);
                });

            // Buttons to change the language
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    for locale in ["en-US", "de", "fr"] {
                        parent
                            .spawn_bundle(get_locale_button_bundle())
                            .with_children(|parent| {
                                parent.spawn_bundle(get_button_text_bundle(locale, font.clone()));
                            })
                            .insert(LanguageButton(locale));
                    }
                });
        });
}

fn get_locale_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

fn get_button_text_bundle(value: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            value,
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::BLACK,
            },
        ),
        ..default()
    }
}
