use std::path::{Path, PathBuf};

use unic_langid::LanguageIdentifier;

use crate::locale::{Locale, LocaleDefaultFallback, LocaleFallbackMap};

use super::LocalizationFolder;

/// Get the path of the FTL file for the given language ID in the localization folder.
pub fn get_ftl_path<T: LocalizationFolder>(lang_id: &LanguageIdentifier) -> PathBuf {
    Path::new(&T::FOLDER_PATH).join(format!("{}.ftl", lang_id))
}

pub fn get_resolution_chain(
    locale: &Locale,
    fallback_map: &LocaleFallbackMap,
    default_fallback: &LocaleDefaultFallback,
) -> Vec<LanguageIdentifier> {
    let locale = locale.0.clone();
    let fallbacks = fallback_map.0.get(&locale);
    let default_fallback = default_fallback.0.clone();

    let mut resolution_chain = Vec::with_capacity(fallbacks.map_or(0, |f| f.len()) + 2);

    resolution_chain.push(locale);

    if let Some(fallbacks) = fallbacks {
        resolution_chain.append(&mut fallbacks.clone());
    }

    if let Some(default_fallback) = default_fallback {
        resolution_chain.push(default_fallback);
    }

    resolution_chain
}
