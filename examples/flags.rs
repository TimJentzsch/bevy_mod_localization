use bevy::prelude::*;
use bevy_mod_localization::locale::{Locale, LocaleId};
use bevy_mod_localization::plugin::LocalizationPlugin;

#[derive(Component)]
struct LanguageButton(LocaleId);

fn main() {
    App::new()
        .add_plugins((LocalizationPlugin, DefaultPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, interaction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            // Buttons
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(10.),
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    let languages = ["en-US", "en-GB", "fr", "de"];
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

                    // A button to switch to each language
                    for language in languages {
                        builder
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(150.),
                                        border: UiRect::all(Val::Px(2.)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::WHITE),
                                    ..default()
                                },
                                LanguageButton(language.parse().unwrap()),
                            ))
                            .with_children(|builder| {
                                builder.spawn(TextBundle::from_section(
                                    language,
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 40.,
                                        color: Color::WHITE,
                                    },
                                ));
                            });
                    }
                });

            // Flag image
            builder.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(90.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("localized://flag.png")),
            ));
        });
}

fn interaction(
    mut interaction_query: Query<
        (&Interaction, &LanguageButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut locale: ResMut<Locale>,
) {
    for (interaction, language_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Update the locale to the one that the user pressed on
                locale.locale_id = language_button.0.clone();
            }
            _ => {}
        }
    }
}
