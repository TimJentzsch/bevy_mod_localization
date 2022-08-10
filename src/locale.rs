use std::str::FromStr;

use unic_langid::LanguageIdentifier;

/// The currently active locale
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Locale(pub(crate) LanguageIdentifier);

impl Locale {
    pub fn new<T: IntoLanguageIdentifier>(locale: T) -> Self {
        Locale(locale.into_language_identifier())
    }

    pub fn update<T: IntoLanguageIdentifier>(&mut self, locale: T) {
        self.0 = locale.into_language_identifier();
    }
}

pub trait IntoLanguageIdentifier {
    fn into_language_identifier(self) -> LanguageIdentifier;
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
