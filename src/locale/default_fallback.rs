use unic_langid::LanguageIdentifier;

use super::into_language_identifier::IntoLanguageIdentifier;

/// The default locale to fall back to.
///
/// Locale resolution: [`Locale`] -> [`LocaleFallbackMap`] -> [`LocaleDefaultFallback`].
#[derive(Debug, Default)]
pub struct LocaleDefaultFallback(pub(crate) Option<LanguageIdentifier>);

impl LocaleDefaultFallback {
    /// Create a new [`LocaleDefaultFallback`].
    ///
    /// The [`LocaleDefaultFallback`] should be inserted as a resource into your app:
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_prototype_fluent::prelude::*;
    /// #
    /// App::new()
    ///     .insert_resource(LocaleDefaultFallback::new(Some("en-US")))
    ///     .insert_resource(Locale::new("en-GB"))
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
    /// # use bevy_prototype_fluent::prelude::*;
    /// use unic_langid::langid;
    ///
    /// App::new()
    ///     .insert_resource(LocaleDefaultFallback::new(Some(langid!("en-US"))))
    ///     .add_plugin(LocalizationPlugin)
    ///     // -- snip --
    ///     .run();
    /// ```
    pub fn new<T: IntoLanguageIdentifier>(locale: Option<T>) -> Self {
        Self(locale.map(|into_locale| into_locale.into_language_identifier()))
    }

    /// Change the default fallback locale.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_prototype_fluent::prelude::*;
    /// #
    /// fn change_default_locale(mut default_locale: ResMut<LocaleDefaultFallback>) {
    ///     // Change the default fallback locale to French
    ///     // This means that if a string is not defined for the current [`Locale`]
    ///     // and no other fallback is specified in the [`LocaleFallbackMap`],
    ///     // then the French string will be taken instead (if available)
    ///     default_locale.set(Some("fr"));
    /// }
    /// ```
    pub fn set<T: IntoLanguageIdentifier>(&mut self, locale: Option<T>) {
        self.0 = locale.map(|into_locale| into_locale.into_language_identifier());
    }
}
