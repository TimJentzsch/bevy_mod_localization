use std::{fmt::Display, str::FromStr};

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct LanguageId(unic_langid::LanguageIdentifier);

impl Display for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<F> From<F> for LanguageId
where
    F: Into<unic_langid::LanguageIdentifier>,
{
    fn from(value: F) -> Self {
        Self(value.into())
    }
}

impl FromStr for LanguageId {
    type Err = <unic_langid::LanguageIdentifier as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
