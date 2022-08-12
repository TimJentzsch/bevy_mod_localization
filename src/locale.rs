use std::str::FromStr;

use bevy::utils::HashMap;
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
/// # use bevy_prototype_fluent::prelude::*;
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
/// # use bevy_prototype_fluent::prelude::*;
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
/// # use bevy_prototype_fluent::prelude::*;
/// #
/// fn change_locale(mut locale: ResMut<Locale>) {
///     // Change the locale to French
///     locale.set("fr");
/// }
/// ```
///
/// Don't forget to add the system to your app.
/// This method can be easily expanded to change the locale on a button press or other user input.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Locale(pub(crate) LanguageIdentifier);

impl Locale {
    /// Create a new [`Locale`].
    ///
    /// The [`Locale`] should be inserted as a resource into your app:
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_prototype_fluent::prelude::*;
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
    /// # use bevy_prototype_fluent::prelude::*;
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
    /// # use bevy_prototype_fluent::prelude::*;
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

/// A map of locales to fall back to.
///
/// For example, if a string is not defined for `en-GB`, then you might want to use the string
/// from `en-US` instead, if available.
///
/// Locale resolution: [`Locale`] -> [`LocaleFallbackMap`] -> [`LocaleDefaultFallback`].
#[derive(Debug, Default)]
pub struct LocaleFallbackMap(pub(crate) HashMap<LanguageIdentifier, Vec<LanguageIdentifier>>);

impl LocaleFallbackMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the fallbacks for the given locale.
    ///
    /// The order of the fallbacks matters, they will be tried first to last.
    pub fn insert<K, V>(&mut self, locale: K, fallbacks: Vec<V>) -> Option<Vec<LanguageIdentifier>>
    where
        K: IntoLanguageIdentifier,
        V: IntoLanguageIdentifier,
    {
        let key = locale.into_language_identifier();
        let value: Vec<LanguageIdentifier> = fallbacks
            .into_iter()
            .map(|x| x.into_language_identifier())
            .collect();

        self.0.insert(key, value)
    }

    /// Add a new fallback language to the given locale.
    ///
    /// If the new fallback already existed for the locale, nothing happens.
    /// Otherwise, it will be added at the _end_ of the fallback list.
    pub fn add_fallback<K, V>(&mut self, locale: K, fallback: V)
    where
        K: IntoLanguageIdentifier,
        V: IntoLanguageIdentifier,
    {
        let key = locale.into_language_identifier();
        let value = fallback.into_language_identifier();

        let cur_fallbacks = self.0.get_mut(&key);

        if let Some(cur_fallbacks) = cur_fallbacks {
            if !cur_fallbacks.contains(&value) {
                cur_fallbacks.push(value);
            }
        } else {
            self.insert(key, vec![value]);
        }
    }
}

/// A helper trait to reduce boilerplate when creating a new [`Locale`].
pub trait IntoLanguageIdentifier {
    fn into_language_identifier(self) -> LanguageIdentifier;
}

impl IntoLanguageIdentifier for LanguageIdentifier {
    fn into_language_identifier(self) -> LanguageIdentifier {
        self
    }
}

impl IntoLanguageIdentifier for &str {
    fn into_language_identifier(self) -> LanguageIdentifier {
        LanguageIdentifier::from_str(self).expect("Invalid language ID")
    }
}

impl IntoLanguageIdentifier for String {
    fn into_language_identifier(self) -> LanguageIdentifier {
        LanguageIdentifier::from_str(self.as_str()).expect("Invalid language ID")
    }
}
