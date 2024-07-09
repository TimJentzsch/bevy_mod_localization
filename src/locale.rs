use std::str::FromStr;

use bevy::prelude::Resource;
use unic_langid::LanguageIdentifier;

/// The identifier of a specific locale, e.g. `en-US` or `fr`.
///
/// Follows the Unicode Language Identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleId {
    language_id: LanguageIdentifier,
}

impl LocaleId {
    pub fn language_id(&self) -> &LanguageIdentifier {
        &self.language_id
    }
}

impl FromStr for LocaleId {
    type Err = <LanguageIdentifier as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LanguageIdentifier::from_str(s).map(|language_id| Self { language_id })
    }
}

/// The currently active locale.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct Locale {
    pub locale_id: LocaleId,
}
