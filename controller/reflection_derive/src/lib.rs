/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Item};

struct DerivedReflect {
    pub comment: Option<String>
}

impl DerivedReflect {
    fn implementation(self, ident: syn::Ident) -> TokenStream {
        let ty = quote!(#ident);
        let test_name = format_ident!(
            "export_bindings_{}",
            ty.to_string().to_lowercase().replace("r#", "")
        );

        quote! {
            impl reflection::Reflect for #ty {
                const JSDOC_COMMENT: Option<&'static str> = None;

                fn ts_definition() -> String {
                    String::from("any")
                }
            }

            #[cfg(test)]
            #[test]
            fn #test_name() {
                #ty::export_all().expect("could not export type");
            }
        }
    }
}

/// Options for the derive macro.
#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(my_trait))]
struct DeriveOptions {
    
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Unsupported use: {0}")]
    UnsupportedUse(&'static str)
}

/// A derive macro that generates a test to create reflection bindings for the specified type.
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn derive_reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input);
    let options = DeriveOptions::from_derive_input(&derive_input).expect("Wrong options");

    let span = derive_input.span();
    match reflect(derive_input) {
        Err(err) => syn::Error::new(span, err.to_string()).to_compile_error().into(),
        Ok(output) => output.into(),
    }
}

fn reflect(ast: DeriveInput) -> Result<TokenStream, Error> {
    let ident = ast.ident;
    match &ast.data {
        syn::Data::Struct(s) => Ok(struct_definition(s)),
        syn::Data::Enum(e) => Ok(enum_definition(e)),
        _ => Err(Error::UnsupportedUse("only structs and enums are supported")),
    }?.map(|reflect| reflect.implementation(ident))
}

fn struct_definition(s: &syn::DataStruct) -> Result<DerivedReflect, Error> {
    // TODO
    Ok(DerivedReflect {
        comment: None
    })
}

fn enum_definition(e: &syn::DataEnum) -> Result<DerivedReflect, Error> {
    // TODO
    Ok(DerivedReflect {
        comment: None
    })
}