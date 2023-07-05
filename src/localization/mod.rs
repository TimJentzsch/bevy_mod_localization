mod asset;
mod resolution_chain;
mod systems;
mod text;
mod utils;

pub use text::LocalizedText;

use crate::{fluent::FluentBundle, LocalizationSource};
use bevy::{prelude::*, utils::HashMap};
use fluent::FluentArgs;
use unic_langid::LanguageIdentifier;

use crate::LocalizationError;

#[derive(Resource)]
pub struct Localization {
    pub(crate) handle_map: HashMap<LanguageIdentifier, Handle<LocalizationSource>>,
    pub(crate) bundle_map: HashMap<LanguageIdentifier, FluentBundle>,
    pub(crate) resolution_chain: Vec<LanguageIdentifier>,
}

impl Localization {
    pub fn new(resolution_chain: Vec<LanguageIdentifier>) -> Self {
        Self {
            resolution_chain,
            ..Default::default()
        }
    }

    fn try_format_pattern(
        &self,
        message_id: &str,
        args: Option<&FluentArgs>,
    ) -> Result<String, LocalizationError> {
        for lang_id in &self.resolution_chain {
            if let Some(bundle) = self.bundle_map.get(lang_id) {
                if let Some(msg) = bundle.get_message(message_id) {
                    let mut errors = vec![];

                    if let Some(pattern) = msg.value() {
                        let formatted_message = bundle.format_pattern(pattern, args, &mut errors);

                        if errors.is_empty() {
                            return Ok(formatted_message.to_string());
                        }
                    }
                }
            }
        }

        Err(LocalizationError)
    }

    pub fn try_get_message(&self, message_id: &str) -> Result<String, LocalizationError> {
        self.try_format_pattern(message_id, None)
    }

    pub fn try_format_message(
        &self,
        message_id: &str,
        args: FluentArgs,
    ) -> Result<String, LocalizationError> {
        self.try_format_pattern(message_id, Some(&args))
            // The inserted values are wrapped in U+2058 (First Strong Isolate) and U+2069 (Pop Directional Isolate)
            // The font can't handle them, so we replace them for now
            // TODO: Don't do this
            .map(|msg| msg.replace(['\u{2068}', '\u{2069}'], ""))
    }
}
