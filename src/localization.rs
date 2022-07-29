use std::{marker::PhantomData, path::Path};

use crate::{fluent::FluentBundle, CurrentLocale, LocalizationSource};
use bevy::prelude::*;

use crate::LocalizationError;

// TODO: Review if the 'static is really needed for world.contains_resource
pub trait LocalizationFolder: 'static + std::marker::Send + std::marker::Sync {
    fn folder_path() -> String;
}

pub struct Localization<T: LocalizationFolder> {
    phantom: std::marker::PhantomData<T>,
    handle: Handle<LocalizationSource>,
    cur_bundle: Option<FluentBundle>,
}

impl<T: LocalizationFolder> Localization<T> {
    pub fn new(handle: Handle<LocalizationSource>) -> Self {
        Self {
            phantom: PhantomData,
            handle,
            cur_bundle: None,
        }
    }

    pub fn try_get_message(&self, message_id: &str) -> Result<String, LocalizationError> {
        let bundle = if let Some(bundle) = &self.cur_bundle {
            bundle
        } else {
            return Err(LocalizationError);
        };

        let msg = bundle
            .get_message(message_id)
            .expect("Message doesn't exist.");

        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(pattern, None, &mut errors);

        if !errors.is_empty() {
            Err(LocalizationError)
        } else {
            Ok(value.to_string())
        }
    }
}

pub trait AddLocalization {
    fn add_localization<T: LocalizationFolder>(&mut self) -> &mut Self;
}

impl AddLocalization for App {
    fn add_localization<T: LocalizationFolder>(&mut self) -> &mut Self {
        if self.world.contains_resource::<Localization<T>>() {
            return self;
        }

        let locale = self.world.resource::<CurrentLocale>();
        let asset_server = self.world.resource::<AssetServer>();

        let ftl_path = Path::new(&T::folder_path()).join(format!("{}.ftl", locale.0.to_string()));
        let handle: Handle<LocalizationSource> = asset_server.load(ftl_path);

        let localization = Localization::<T>::new(handle);

        self.insert_resource(localization);

        self
    }
}
