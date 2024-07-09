use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use async_fs::{read_dir, File};
use bevy::asset::io::{AssetReader, AssetReaderError, PathStream, Reader};
use futures_lite::StreamExt;

use crate::locale::Locale;

/// An asset reader which will consider the currently active locale.
///
/// For example, when the locale `fr` is active, an asset path like `/flag.png` will instead be resolved to `/flag/fr.png`.
pub struct LocalizedReader {
    /// The root to use for resolving asset paths.
    root_path: PathBuf,

    /// The locale which is currently active.
    locale: Locale,
}

// HACK: Implementation mostly copy & pasted from Bevy's `FileAssetReader`.
// I tried wrapping the existing implementation, but ran into lifetime issues that I couldn't resolve.
// (We need to pass a modified `path` to `read`, but then a reference to a local variable is returned)
impl AssetReader for LocalizedReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let full_path = self.root_path.join(path);
        let localized_bath = get_localized_path(&full_path, &self.locale);

        File::open(&localized_bath).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AssetReaderError::NotFound(localized_bath)
            } else {
                e.into()
            }
        })
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let meta_path = get_meta_path(path);
        let full_path = self.root_path.join(meta_path);
        File::open(&full_path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AssetReaderError::NotFound(full_path)
            } else {
                e.into()
            }
        })
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        let full_path = self.root_path.join(path);
        match read_dir(&full_path).await {
            Ok(read_dir) => {
                let root_path = self.root_path.clone();
                let mapped_stream = read_dir.filter_map(move |f| {
                    f.ok().and_then(|dir_entry| {
                        let path = dir_entry.path();
                        // filter out meta files as they are not considered assets
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if ext.eq_ignore_ascii_case("meta") {
                                return None;
                            }
                        }
                        let relative_path = path.strip_prefix(&root_path).unwrap();
                        Some(relative_path.to_owned())
                    })
                });
                let read_dir: Box<PathStream> = Box::new(mapped_stream);
                Ok(read_dir)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err(AssetReaderError::NotFound(full_path))
                } else {
                    Err(e.into())
                }
            }
        }
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        let full_path = self.root_path.join(path);
        let metadata = full_path
            .metadata()
            .map_err(|_e| AssetReaderError::NotFound(path.to_owned()))?;
        Ok(metadata.file_type().is_dir())
    }
}

/// Get the path of the directory which contains the localized assets.
///
/// `/flags.png` -> `/flags`
fn get_localized_dir_path(path: &Path) -> PathBuf {
    let mut localized_dir_path = path.to_path_buf();
    localized_dir_path.set_extension(OsString::new());
    localized_dir_path
}

/// Transform a path into its localized version for the given locale.
///
/// `flags.png` -> `/flags/en-US.png`
fn get_localized_path(path: &Path, locale: &Locale) -> PathBuf {
    let extension = path.extension().unwrap_or_default().to_os_string();
    let mut localized_path = get_localized_dir_path(path);

    localized_path.push(locale.language_id().to_string());
    localized_path.set_extension(extension);

    localized_path
}

/// Get the path of the meta file of a localized asset.
///
/// `/flags.png` -> `/flags/.meta`
fn get_meta_path(path: &Path) -> PathBuf {
    let mut meta_path = get_localized_dir_path(path);
    meta_path.push(".meta");
    meta_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_localized_dir_path() {
        let path = PathBuf::from("/flag.png");
        let localized_dir_path = get_localized_dir_path(&path);

        assert_eq!(localized_dir_path, PathBuf::from("/flag"));
    }

    #[test]
    fn test_get_localized_path() {
        let path = PathBuf::from("/flag.png");
        let locale = "en-US".parse().unwrap();
        let localized_path = get_localized_path(&path, &locale);

        assert_eq!(localized_path, PathBuf::from("/flag/en-US.png"));
    }

    #[test]
    fn test_get_meta_path() {
        let path = PathBuf::from("/flag.png");
        let meta_path = get_meta_path(&path);

        assert_eq!(meta_path, PathBuf::from("/flag/.meta"));
    }
}
