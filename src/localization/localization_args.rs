use std::marker::PhantomData;

use bevy::prelude::*;
use fluent::FluentArgs;

use super::{LocalizationArg, LocalizationVariables};

pub struct LocalizationArgs;

impl LocalizationVariables for LocalizationArgs {
    fn get_fluent_args(&self, _world: &World) -> FluentArgs {
        FluentArgs::new()
    }

    fn has_any_variable_changed(&self, _world: &World) -> bool {
        false
    }
}

impl LocalizationArgs {
    pub fn new() -> Self {
        Self
    }

    pub fn add_arg<A>(&mut self, id: &'static str) -> LocalizationArgs1<A>
    where
        A: LocalizationArg,
    {
        LocalizationArgs1 {
            _arg_1: PhantomData,
            id_1: id,
        }
    }
}

impl Default for LocalizationArgs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LocalizationArgs1<A1>
where
    A1: LocalizationArg,
{
    _arg_1: PhantomData<A1>,
    id_1: &'static str,
}

impl<A1> LocalizationVariables for LocalizationArgs1<A1>
where
    A1: LocalizationArg,
{
    fn get_fluent_args(&self, world: &World) -> FluentArgs {
        let mut args = FluentArgs::new();

        args.set(self.id_1, A1::localization_value(world));

        args
    }

    fn has_any_variable_changed(&self, world: &World) -> bool {
        A1::has_changed(world)
    }
}

impl<A1> LocalizationArgs1<A1>
where
    A1: LocalizationArg,
{
    pub fn add_arg<A>(&mut self, id: &'static str) -> LocalizationArgs2<A1, A>
    where
        A: LocalizationArg,
    {
        LocalizationArgs2 {
            _arg_1: self._arg_1,
            id_1: self.id_1,
            _arg_2: PhantomData,
            id_2: id,
        }
    }
}

pub struct LocalizationArgs2<A1, A2>
where
    A1: LocalizationArg,
    A2: LocalizationArg,
{
    _arg_1: PhantomData<A1>,
    id_1: &'static str,

    _arg_2: PhantomData<A2>,
    id_2: &'static str,
}

impl<A1, A2> LocalizationVariables for LocalizationArgs2<A1, A2>
where
    A1: LocalizationArg,
    A2: LocalizationArg,
{
    fn get_fluent_args(&self, world: &World) -> FluentArgs {
        let mut args = FluentArgs::new();

        args.set(self.id_1, A1::localization_value(world));
        args.set(self.id_2, A2::localization_value(world));

        args
    }

    fn has_any_variable_changed(&self, world: &World) -> bool {
        A1::has_changed(world) || A2::has_changed(world)
    }
}
