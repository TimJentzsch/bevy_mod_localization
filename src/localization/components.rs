use std::marker::PhantomData;

use bevy::prelude::*;

use super::LocalizationFolder;

/// Automatically localize an entity with a [`Text`] component.
///
/// The first section of the [`Text`] component will be updated with the specified message.
/// This will be updated every time the locale or localization file changes.
#[derive(Component)]
pub struct LocalizedText<T: LocalizationFolder> {
    phantom: PhantomData<T>,
    message_id: &'static str,
}

impl<T: LocalizationFolder> LocalizedText<T> {
    /// Create a new localzed text with the given message ID.
    pub fn new(message_id: &'static str) -> Self {
        Self {
            phantom: PhantomData,
            message_id,
        }
    }

    /// Get the message ID of the localized text.
    pub fn message_id(&self) -> &'static str {
        self.message_id
    }
}
