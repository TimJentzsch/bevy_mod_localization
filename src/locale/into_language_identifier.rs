use std::str::FromStr;
use unic_langid::LanguageIdentifier;

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
