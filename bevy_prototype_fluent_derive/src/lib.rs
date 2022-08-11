use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Attribute, DeriveInput, Lit, Meta};

/// Automatically implements the `LocalizationFolder` trait.
///
/// Must be used like this:
///
/// ```
/// #[derive(LocalizationFolder)]
/// #[folder_path = "path/to/folder"]
/// struct ExampleFolder;
/// ````
///
/// The folder path works like an asset path, i.e. by default, `/assets` is the root.
#[proc_macro_derive(LocalizationFolder, attributes(folder_path))]
pub fn localization_folder_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    impl_localization_folder(&ast)
}

/// Implementation of the `LocalizationFolder` derive macro.
fn impl_localization_folder(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let folder_path = get_folder_path(&ast.attrs);

    let gen = quote! {
        impl LocalizationFolder for #name {
            fn folder_path() -> String {
                #folder_path.to_string()
            }
        }
    };
    gen.into()
}

/// Get the folder path.
///
/// This is extracted from a `#[folder_path = "path/to/folder"] attribute.
fn get_folder_path(attributes: &[Attribute]) -> String {
    for attribute in attributes.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let name_value = if let Meta::NameValue(name_value) = attribute {
            name_value
        } else {
            continue;
        };

        // Find the attribute "folder_path"
        if name_value
            .path
            .get_ident()
            .map(|i| i != "folder_path")
            .unwrap_or(true)
        {
            continue;
        }

        // Extract the string from the attribute
        let folder_path_str = match name_value.lit {
            Lit::Str(lit_str) => lit_str,
            _ => panic!(
                "`folder_path` attribute must take the form `#[folder_path = \"path/to/folder\"]`."
            ),
        };

        return folder_path_str.value();
    }

    panic!("No #[folder_path = \"path/to/folder\"]` attribute found, required for `#[derive(LocalizationFolder)]`!");
}
