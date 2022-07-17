use anyhow::Result;
use bevy::asset::{AssetLoader, LoadedAsset};
use fluent::FluentResource;

use super::FluentSource;

#[derive(Default)]
pub struct FtlLoader;

impl AssetLoader for FtlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let ftl_string = String::from_utf8(Vec::from(bytes))?;

            let resource =
                FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

            load_context.set_default_asset(LoadedAsset::new(FluentSource::new(resource)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}
