use bevy::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::prelude::{Locale, LocaleDefaultFallback, LocaleFallbackMap};

/// The chain of languages to fall back on, in order.
#[derive(Resource)]
pub struct LocaleResolutionChain {
    pub(crate) chain: Vec<LanguageIdentifier>,
}

/// Update the resolution chain when the locale changes.
pub fn update_resolution_chain(
    mut resolution_chain: ResMut<LocaleResolutionChain>,
    locale: Res<Locale>,
    fallback_map: Res<LocaleFallbackMap>,
    default_fallback: Res<LocaleDefaultFallback>,
) {
    if locale.is_changed() || fallback_map.is_changed() || default_fallback.is_changed() {
        let locale = locale.0.clone();
        let fallbacks = fallback_map.0.get(&locale);
        let default_fallback = default_fallback.0.clone();

        let mut chain = Vec::with_capacity(fallbacks.map_or(0, |f| f.len()) + 2);

        chain.push(locale);

        if let Some(fallbacks) = fallbacks {
            chain.append(&mut fallbacks.clone());
        }

        if let Some(default_fallback) = default_fallback {
            chain.push(default_fallback);
        }

        resolution_chain.chain = chain;
    }
}
