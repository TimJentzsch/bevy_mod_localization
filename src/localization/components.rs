use std::marker::PhantomData;

use bevy::prelude::*;
use fluent::FluentArgs;

use super::{localization_args::LocalizationArgs, LocalizationFolder, LocalizationVariables};

/// Automatically localize an entity with a [`Text`] component.
///
/// The first section of the [`Text`] component will be updated with the specified message.
/// This will be updated every time the locale or localization file changes.
#[derive(Component)]
pub struct LocalizedText<F, V>
where
    F: LocalizationFolder,
    V: LocalizationVariables,
{
    _localization_folder: PhantomData<F>,
    message_id: &'static str,
    variables: V,
}

impl<F> LocalizedText<F, LocalizationArgs>
where
    F: LocalizationFolder,
{
    /// Create a new localzed text with the given message ID.
    pub fn new(message_id: &'static str) -> Self {
        Self {
            _localization_folder: PhantomData,
            message_id,
            variables: LocalizationArgs::new(),
        }
    }
}

impl<F, V> LocalizedText<F, V>
where
    F: LocalizationFolder,
    V: LocalizationVariables,
{
    /// Create a new localzed text with the given message ID and formatting variables.
    pub fn new_with_variables(message_id: &'static str, variables: V) -> Self {
        Self {
            _localization_folder: PhantomData,
            message_id,
            variables,
        }
    }

    /// Get the message ID of the localized text.
    pub fn message_id(&self) -> &'static str {
        self.message_id
    }

    /// Has any argument value changed since last time?
    pub fn has_any_arg_changed(&self, world: &World) -> bool {
        self.variables.has_any_variable_changed(world)
    }

    /// Get the [`FluentArgs`] given the current world state.
    pub fn fluent_args(&self, world: &World) -> FluentArgs {
        self.variables.get_fluent_args(world)
    }
}
