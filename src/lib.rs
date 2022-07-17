use fluent::FluentBundle;

use bevy::reflect::TypeUuid;
use fluent::FluentResource;

mod loaders;

#[derive(Debug, TypeUuid)]
#[uuid = "c807fa98-31ad-4d85-8988-ab4313cced3f"]
pub struct LocalizationSource {
    pub resource: FluentResource,
}

impl LocalizationSource {
    pub fn new(resource: FluentResource) -> Self {
        Self { resource }
    }
}

pub trait LocalizationBundle<R> {
    fn bundle(&self) -> FluentBundle<R>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
