use std::marker::PhantomData;

use bevy::prelude::*;

use super::LocalizationFolder;

#[derive(Component)]
pub struct LocalizedText<T: LocalizationFolder> {
    phantom: PhantomData<T>,
    message_id: &'static str,
}

impl<T: LocalizationFolder> LocalizedText<T> {
    pub fn new(message_id: &'static str) -> Self {
        Self {
            phantom: PhantomData,
            message_id,
        }
    }

    pub fn message_id(&self) -> &'static str {
        self.message_id
    }
}
