use std::path::{Path, PathBuf};

use bevy::{
    asset::io::{file::FileAssetReader, AssetReader, AssetReaderError, PathStream, Reader},
    utils::BoxedFuture,
};

use crate::core::{language_id::LanguageId, locale_chain::LocaleChain};

struct LocalizedAssetReader {
    internal_reader: FileAssetReader,
    locale_chain: LocaleChain,
}

impl LocalizedAssetReader {
    pub fn localized_path<'a>(locale: &LanguageId, path: &'a Path) -> PathBuf {
        let base_path = path.parent().unwrap();
        let extension = path.extension().unwrap();
        let stem = path.file_stem().unwrap();

        let mut localized_path = base_path.to_path_buf();
        localized_path.push(stem);
        localized_path.push(format!(
            "{locale}.{}",
            extension
                .to_str()
                .expect("Failed to convert extension to string")
        ));

        localized_path
    }

    pub fn localized_path_chain<'a>(&self, path: &'a Path) -> Vec<PathBuf> {
        self.locale_chain
            .0
            .iter()
            .map(|locale| Self::localized_path(locale, path))
            .collect()
    }
}

impl AssetReader for LocalizedAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let path_chain = self.localized_path_chain(path);

            for path in path_chain {
                match self.internal_reader.read(&path).await {
                    // If the path does not exist, move on to the next locale
                    Err(AssetReaderError::NotFound(_)) => continue,
                    // Otherwise, return the result
                    res => return res,
                }
            }

            // No locale could be found
            Err(AssetReaderError::NotFound(path.to_owned()))
        })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let path_chain = self.localized_path_chain(path);

            // TODO: "Sync" which file and meta file is selected
            for path in path_chain {
                match self.internal_reader.read_meta(&path).await {
                    // If the path does not exist, move on to the next locale
                    Err(AssetReaderError::NotFound(_)) => continue,
                    // Otherwise, return the result
                    res => return res,
                }
            }

            // No locale could be found
            Err(AssetReaderError::NotFound(path.to_owned()))
        })
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        // TODO: Maybe we need to handle the locale here too?
        self.internal_reader.is_directory(path)
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        // TODO: Maybe we need to handle the locale here too?
        self.internal_reader.read_directory(path)
    }
}
