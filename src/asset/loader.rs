use anyhow::Result;
use bevy::asset::{io::Reader, AssetLoader, AsyncReadExt as _, LoadContext};

use crate::LocalizationSource;

#[derive(Default)]
pub struct FtlLoader;

impl AssetLoader for FtlLoader {
    type Asset = LocalizationSource;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let ftl_string = String::from_utf8(bytes)?;

            Ok(LocalizationSource::new(ftl_string))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}
