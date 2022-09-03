use std::marker::PhantomData;

use bevy::prelude::*;
use fluent::FluentValue;

use super::LocalizationFolder;

pub trait LocalizationArg: Send + Sync {
    fn localization_value<'a>(&self, world: &World) -> FluentValue<'a>;
}

struct LocalizationVariable {
    id: &'static str,
    arg: Box<dyn LocalizationArg>,
}

impl LocalizationVariable {
    pub fn new<A>(id: &'static str, arg: A) -> Self
    where
        // TODO: Do we need 'static here?
        A: LocalizationArg + 'static,
    {
        Self {
            id,
            arg: Box::new(arg),
        }
    }
}

/// Automatically localize an entity with a [`Text`] component.
///
/// The first section of the [`Text`] component will be updated with the specified message.
/// This will be updated every time the locale or localization file changes.
#[derive(Component)]
pub struct LocalizedText<F>
where
    F: LocalizationFolder,
{
    _localization_folder: PhantomData<F>,
    message_id: &'static str,
    args: Vec<LocalizationVariable>,
}

impl<F> LocalizedText<F>
where
    F: LocalizationFolder,
{
    /// Create a new localzed text with the given message ID.
    pub fn new(message_id: &'static str) -> Self {
        Self {
            _localization_folder: PhantomData,
            message_id,
            args: Vec::new(),
        }
    }

    /// Get the message ID of the localized text.
    pub fn message_id(&self) -> &'static str {
        self.message_id
    }

    pub fn add_arg<A>(&mut self, id: &'static str, arg: A) -> &mut LocalizedText<F>
    where
        // TODO: Do we need 'static here?
        A: LocalizationArg + 'static,
    {
        self.args.push(LocalizationVariable::new(id, arg));
        self
    }
}
