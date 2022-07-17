pub mod ftl_loader;

use bevy::reflect::TypeUuid;
use fluent::FluentResource;

#[derive(Debug, TypeUuid)]
#[uuid = "c807fa98-31ad-4d85-8988-ab4313cced3f"]
pub struct FluentSource {
    pub resource: FluentResource,
}

impl FluentSource {
    pub fn new(resource: FluentResource) -> FluentSource {
        FluentSource { resource }
    }
}
