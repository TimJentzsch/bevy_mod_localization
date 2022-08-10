use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_prototype_fluent::prelude::*;
use fluent::FluentArgs;

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

struct InteractiveLocalizationFolder;

// TODO: Write a derive macro for this
impl LocalizationFolder for InteractiveLocalizationFolder {
    fn folder_path() -> String {
        "strings/interactive".to_string()
    }
}

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
struct EnglishButton;

#[derive(Component)]
struct GermanButton;

#[derive(Component)]
struct FrenchButton;

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
    mut current_locale: ResMut<Locale>,
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
                    "en-US"
                } else if let Some(_) = german {
                    "de"
                } else if let Some(_) = french {
                    "fr"
                } else {
                    continue;
                };

                current_locale.update(language_id);
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
            // Node for simple text
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        // This will later be replaced by the localized text
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(WelcomeText);

            // Node for parameterized text
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        // This will later be replaced by the localized text
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..default()
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
                        text: Text::with_section(
                            "Counter: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..default()
                            },
                        ),
                        ..default()
                    });

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle("+", asset_server.clone()));
                        })
                        .insert(CountIncrementButton);

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle("-", asset_server.clone()));
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
                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle(
                                "English",
                                asset_server.clone(),
                            ));
                        })
                        .insert(EnglishButton);

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle(
                                "Deutsch",
                                asset_server.clone(),
                            ));
                        })
                        .insert(GermanButton);

                    parent
                        .spawn_bundle(get_locale_button_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(get_button_text_bundle(
                                "Français",
                                asset_server.clone(),
                            ));
                        })
                        .insert(FrenchButton);
                });
        });
}

fn get_locale_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

fn get_button_text_bundle(value: &str, asset_server: AssetServer) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            },
            Default::default(),
        ),
        ..default()
    }
}
