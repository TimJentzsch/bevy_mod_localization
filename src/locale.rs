use std::str::FromStr;

use unic_langid::LanguageIdentifier;

/// A specific locale, e.g. `en-US` or `fr`.
///
/// Follows the Unicode Language Identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    language_id: LanguageIdentifier,
}

impl Locale {
    pub fn language_id(&self) -> &LanguageIdentifier {
        &self.language_id
    }
}

impl FromStr for Locale {
    type Err = <LanguageIdentifier as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LanguageIdentifier::from_str(s).map(|language_id| Self { language_id })
    }
}
