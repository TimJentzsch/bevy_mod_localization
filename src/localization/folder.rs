/// A folder containing localization files.
///
/// This trait is used to define the path to a localization folder.
/// The path works like asset paths, i.e. by default, `asset` is the root of the path.
/// Instead of implementing it manually, it is recommended to use the derive macro:
///
/// ```
/// # use bevy_prototype_fluent::prelude::*;
/// #
/// #[derive(LocalizationFolder)]
/// #[folder_path = "/strings/example"]
/// struct ExampleLocalizationFolder;
///
/// assert_eq!(ExampleLocalizationFolder::FOLDER_PATH, "/strings/example");
/// ```
///
/// By default, this will point to the folder `/assets/strings/example` in your crate.
///
/// The folder should then contain `.ftl` files for each language you want to support.
/// The files must be named after the corresponding unicode language tag they represent.
/// The following structure corresponds to the example above:
///
/// ```txt
/// my_crate/
/// ├─ assets/
/// │  ├─ strings/
/// │  │  ├─ example/
/// │  │  │  ├─ en-US.ftl
/// │  │  │  ├─ de.ftl
/// │  │  │  ├─ fr.ftl
/// ```
// TODO: Review if the 'static is really needed for world.contains_resource
pub trait LocalizationFolder: 'static + std::marker::Send + std::marker::Sync {
    const FOLDER_PATH: &'static str;
}
