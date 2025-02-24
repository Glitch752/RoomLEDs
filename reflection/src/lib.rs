/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Item};

mod export;

trait TypeVisitor: Sized {
    fn visit<T: Reflect + 'static + ?Sized>(&mut self);
}

trait Reflect {
    const JSDOC_COMMENT: Option<&'static str> = None;

    fn ts_definition() -> String;
    
    fn visit_dependencies(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }
}

struct DerivedReflect {
}

impl DerivedReflect {
    fn implementation(self, ident: syn::Ident) -> TokenStream {
        quote! {
            // TODO
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Syn error: {0}")]
    Syn(#[from] syn::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("The environment variable CARGO_MANIFEST_DIR must be set.")]
    ManifestDirNotSet,

    #[error("This use is unsupported: {0}")]
    UnsupportedUse(&'static str),
}

/// A derive macro that generates a test to create reflection bindings for the specified type.
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn derive_reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = match syn::parse::<Item>(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };
    let span = ast.span();
    match reflect(ast) {
        Err(err) => syn::Error::new(span, err.to_string()).to_compile_error().into(),
        Ok(output) => output.into(),
    }
}

fn reflect(ast: Item) -> Result<TokenStream, Error> {
    let (reflect, ident) = match ast {
        Item::Struct(s) => (struct_definition(&s)?, s.ident),
        Item::Enum(e) => (enum_definition(&e)?, e.ident),
        _ => return Err(Error::UnsupportedUse("Only structs and enums are supported.")),
    };

    Ok(reflect.implementation(ident))
}

fn struct_definition(s: &syn::ItemStruct) -> Result<DerivedReflect, Error> {
    // TODO
    Ok(DerivedReflect {})
}

fn enum_definition(e: &syn::ItemEnum) -> Result<DerivedReflect, Error> {
    // TODO
    Ok(DerivedReflect {})
}