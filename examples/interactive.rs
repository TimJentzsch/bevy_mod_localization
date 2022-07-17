use bevy::prelude::*;
// TODO: Figure out why importing the prelude doesn't work here
use bevy_asset_loader::*;
use bevy_prototype_fluent::{
    CurrentLocale, LocalizationBundle, LocalizationError, LocalizationPlugin, LocalizationSource,
};
use unic_langid::{langid, LanguageIdentifier};

const EN_US: LanguageIdentifier = langid!("en-US");
const DE: LanguageIdentifier = langid!("de");
const FR: LanguageIdentifier = langid!("fr");

#[derive(AssetCollection)]
struct ExampleLocalization {
    #[asset(path = "strings/interactive/en_us.ftl")]
    en_us: Handle<LocalizationSource>,

    #[asset(path = "strings/interactive/de.ftl")]
    de: Handle<LocalizationSource>,

    #[asset(path = "strings/interactive/fr.ftl")]
    fr: Handle<LocalizationSource>,
}

// TODO: This should be implemented with a fancy derive macro instead
impl LocalizationBundle for ExampleLocalization {
    fn try_get_resource_handle(
        &self,
        language_id: &LanguageIdentifier,
    ) -> Result<Handle<LocalizationSource>, LocalizationError> {
        match *language_id {
            EN_US => Ok(self.en_us.clone()),
            DE => Ok(self.de.clone()),
            FR => Ok(self.fr.clone()),
            _ => Err(LocalizationError),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LocalizationPlugin)
        .insert_resource(CurrentLocale::new(EN_US))
        .init_collection::<ExampleLocalization>()
        .add_startup_system(setup)
        .add_system(text_update_system)
        .add_system(button_system)
        .run();
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

fn button_system(
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
