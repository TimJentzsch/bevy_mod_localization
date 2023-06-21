mod default_fallback;
mod fallback_map;
mod into_language_identifier;

use bevy::prelude::*;
pub use default_fallback::LocaleDefaultFallback;
pub use fallback_map::LocaleFallbackMap;
pub use into_language_identifier::IntoLanguageIdentifier;

use unic_langid::LanguageIdentifier;

/// The currently active locale.
///
/// This struct controls which language your game currently uses.
/// It then determines which `.ftl` file is loaded for the localization.
///
/// # Creating a new [`Locale`]
///
/// The [`Locale`] should be inserted as a resource into your app:
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_mod_localization::prelude::*;
/// #
/// App::new()
///     .insert_resource(Locale::new("en-US"))
///     .add_plugin(LocalizationPlugin)
///     // -- snip --
///     .run();
/// ```
///
/// The strings must be valid unicode language tags.
/// You can also use the [`unic_langid::langid`] macro:
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_mod_localization::prelude::*;
/// use unic_langid::langid;
///
/// App::new()
///     .insert_resource(Locale::new(langid!("en-US")))
///     .add_plugin(LocalizationPlugin)
///     // -- snip --
///     .run();
/// ```
///
/// # Modifying the [`Locale`]
///
/// Modifying the currently active locale is as simple as editing the corresponding resource:
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_mod_localization::prelude::*;
/// #
/// fn change_locale(mut locale: ResMut<Locale>) {
///     // Change the locale to French
///     locale.set("fr");
/// }
/// ```
///
/// Don't forget to add the system to your app.
/// This method can be easily expanded to change the locale on a button press or other user input.
#[derive(PartialEq, Eq, Debug, Clone, Resource)]
pub struct Locale(pub(crate) LanguageIdentifier);

impl Locale {
    /// Create a new [`Locale`].
    ///
    /// The [`Locale`] should be inserted as a resource into your app:
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_mod_localization::prelude::*;
    /// #
    /// App::new()
    ///     .insert_resource(Locale::new("en-US"))
    ///     .add_plugin(LocalizationPlugin)
    ///     // -- snip --
    ///     .run();
    /// ```
    ///
    /// The strings must be valid unicode language tags.
    /// You can also use the [`unic_langid::langid`] macro:
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_mod_localization::prelude::*;
    /// use unic_langid::langid;
    ///
    /// App::new()
    ///     .insert_resource(Locale::new(langid!("en-US")))
    ///     .add_plugin(LocalizationPlugin)
    ///     // -- snip --
    ///     .run();
    /// ```
    pub fn new<T: IntoLanguageIdentifier>(locale: T) -> Self {
        Locale(locale.into_language_identifier())
    }

    /// Change the locale.
    ///
    /// Modifying the currently active locale is as simple as editing the corresponding resource:
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_mod_localization::prelude::*;
    /// #
    /// fn change_locale(mut locale: ResMut<Locale>) {
    ///     // Change the locale to French
    ///     locale.set("fr");
    /// }
    /// ```
    ///
    /// Don't forget to add the system to your app.
    /// This method can be easily expanded to change the locale on a button press or other user input.
    pub fn set<T: IntoLanguageIdentifier>(&mut self, locale: T) {
        self.0 = locale.into_language_identifier();
    }
}
