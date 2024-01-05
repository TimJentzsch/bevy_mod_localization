use bevy::prelude::*;
use bevy::utils::HashMap;

use super::language_id::LanguageId;

/// A map of locales to fall back to.
///
/// For example, if a string is not defined for `en-GB`, then you might want to use the string
/// from `en-US` instead, if available.
///
/// Locale resolution: [`Locale`] -> [`LocaleFallbackMap`] -> [`LocaleDefaultFallback`].
#[derive(Debug, Default, Resource)]
pub struct LocaleFallbackMap(pub(crate) HashMap<LanguageId, Vec<LanguageId>>);

impl LocaleFallbackMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the fallbacks for the given locale.
    ///
    /// The order of the fallbacks matters, they will be tried first to last.
    pub fn insert<K, V>(&mut self, locale: K, fallbacks: Vec<V>) -> Option<Vec<LanguageId>>
    where
        K: Into<LanguageId>,
        V: Into<LanguageId>,
    {
        let key = locale.into();
        let value: Vec<LanguageId> = fallbacks.into_iter().map(|x| x.into()).collect();

        self.0.insert(key, value)
    }

    /// Add a new fallback language to the given locale.
    ///
    /// If the new fallback already existed for the locale, nothing happens.
    /// Otherwise, it will be added at the _end_ of the fallback list.
    pub fn add_fallback<K, V>(&mut self, locale: K, fallback: V)
    where
        K: Into<LanguageId>,
        V: Into<LanguageId>,
    {
        let key = locale.into();
        let value = fallback.into();

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
