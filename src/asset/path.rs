use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::locale::LocaleId;

/// Returns the base path of the assets directory, which is normally the executable's parent
/// directory.
///
/// If the `CARGO_MANIFEST_DIR` environment variable is set, then its value will be used
/// instead. It's set by cargo when running with `cargo run`.
pub fn get_base_path() -> PathBuf {
    if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(manifest_dir)
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| path.parent().map(ToOwned::to_owned).unwrap())
            .unwrap()
    }
}

/// Get the path of the directory which contains the localized assets.
///
/// `/flag.png` -> `/flag`
pub fn get_localized_dir_path(path: &Path) -> PathBuf {
    let mut localized_dir_path = path.to_path_buf();
    localized_dir_path.set_extension(OsString::new());
    localized_dir_path
}

/// Transform a path into its localized version for the given locale.
///
/// `flag.png` -> `/flag/en-US.png`
pub fn get_localized_path(path: &Path, locale: &LocaleId) -> PathBuf {
    let extension = path.extension().unwrap_or_default().to_os_string();
    let mut localized_path = get_localized_dir_path(path);

    localized_path.push(locale.language_id().to_string());
    localized_path.set_extension(extension);

    localized_path
}

/// Get the path of the meta file of a localized asset.
///
/// `/flag.png` -> `/flag/.meta`
pub fn get_meta_path(path: &Path) -> PathBuf {
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
