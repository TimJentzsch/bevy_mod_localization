use bevy::app::{App, Plugin};

use super::{
    default_fallback::LocaleDefaultFallback, fallback_map::LocaleFallbackMap,
    locale_chain::LocaleChain,
};

#[derive(Default)]
pub struct LocalizationCorePlugin;

impl Plugin for LocalizationCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocaleFallbackMap>()
            .init_resource::<LocaleDefaultFallback>()
            .init_resource::<LocaleChain>();
    }
}
