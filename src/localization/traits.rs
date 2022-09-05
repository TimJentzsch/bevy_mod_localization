use bevy::prelude::*;
use fluent::{FluentArgs, FluentValue};

pub trait LocalizationArg: Send + Sync + 'static {
    fn localization_value<'a>(world: &World) -> FluentValue<'a>;
    fn has_changed(world: &World) -> bool;
}

pub trait LocalizationVariables: Sync + Send + 'static {
    fn get_fluent_args(&self, world: &World) -> FluentArgs;
    fn has_any_variable_changed(&self, world: &World) -> bool;
}
