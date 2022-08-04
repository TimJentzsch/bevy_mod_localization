use anyhow::Result;
use bevy::asset::{AssetLoader, LoadedAsset};

use crate::LocalizationSource;

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

            load_context.set_default_asset(LoadedAsset::new(LocalizationSource::new(ftl_string)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}
