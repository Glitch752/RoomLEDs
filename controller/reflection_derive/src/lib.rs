/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

extern crate proc_macro;

use std::rc::Rc;

use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Item, Path, Type};

mod fields;

struct DerivedReflect {
    pub generate_ts_definition: TokenStream,
    pub comment: Option<String>,
    pub dependencies: Vec<Rc<Path>>
}

impl DerivedReflect {
    fn implementation(self, ident: syn::Ident) -> TokenStream {
        let ty = quote!(#ident);
        let test_name = format_ident!(
            "export_bindings_{}",
            ty.to_string().to_lowercase().replace("r#", "")
        );
        let comment_option = match self.comment {
            Some(comment) => quote!(Some(#comment)),
            None => quote!(None)
        };
        let dependencies = self.dependencies.iter().map(|ty| {
            quote! {
                visitor.visit::<#ty>();
                <#ty as reflection::Reflect>::visit_dependencies(visitor);
            }
        }).collect::<TokenStream>();

        let generate_ts_definition = self.generate_ts_definition;

        quote! {
            impl reflection::Reflect for #ty {
                const JSDOC_COMMENT: Option<&'static str> = #comment_option;

                fn ts_definition() -> String {
                    #generate_ts_definition
                }

                fn visit_dependencies(visitor: &mut impl reflection::TypeVisitor) where Self: 'static {
                    #dependencies
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
#[derive(FromDeriveInput, Default, Debug)]
#[darling(default, attributes(reflect), forward_attrs(doc, serde))]
struct ReflectDeriveOptions {
    attrs: Vec<syn::Attribute>
}

// Values from serde's attribute we care about
#[derive(FromMeta, Debug)]
struct SerdeValues {
    tag: Option<String>
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Syn error: {0}")]
    Syn(#[from] syn::Error),
    #[error("Darling error: {0}")]
    Darling(#[from] darling::Error),
    #[error("Unsupported use: {0}")]
    UnsupportedUse(&'static str)
}

/// A derive macro that generates a test to create reflection bindings for the specified type.
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn derive_reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input);
    let options = ReflectDeriveOptions::from_derive_input(&derive_input).expect("Wrong options");

    let span = derive_input.span();
    match reflect(derive_input, options) {
        Err(Error::Darling(err)) => syn::Error::new(err.span(), err.to_string()).to_compile_error().into(),
        Err(Error::Syn(err)) => err.to_compile_error().into(),
        Err(err) => syn::Error::new(span, err.to_string()).to_compile_error().into(),
        Ok(output) => output.into(),
    }
}

fn reflect(derive_input: DeriveInput, options: ReflectDeriveOptions) -> Result<TokenStream, Error> {
    let ast = derive_input.into();
    let (reflect, identifier) = match &ast {
        Item::Struct(s) => Ok((struct_definition(s, options)?, s.ident.clone())),
        Item::Enum(e) => Ok((enum_definition(e, options)?, e.ident.clone())),
        _ => Err(Error::UnsupportedUse("only structs and enums are supported")),
    }?;

    Ok(reflect.implementation(identifier))
}

fn struct_definition(s: &syn::ItemStruct, options: ReflectDeriveOptions) -> Result<DerivedReflect, Error> {
    let doc = options.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("doc") {
            attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value())
        } else {
            None
        }
    });

    // Parse the attributes on each struct field
    let fields = s.fields.iter().filter_map(|field| {
        let attr = match fields::StructFieldAttr::from_field(field) {
            Ok(attr) => attr,
            Err(err) => return Some(Err(err))
        };
        if attr.skip {
            return None;
        }
        Some(Ok((field, attr)))
    }).collect::<Result<Vec<_>, Error>>()?;

    let dependencies = fields.iter().filter_map(|field| {
        if field.1.as_type.is_some() {
            return Some(Rc::new(field.1.as_type.as_ref().unwrap().clone()));
        }
        
        if let Type::Path(path) = &field.0.ty {
            Some(Rc::new(path.path.clone()))
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let field_definitions = fields.iter().map(|(field, attr)| {
        let ts_definition = match &attr.as_type {
            Some(ty) => {
                quote!(<#ty as reflection::Reflect>::inline_ts_definition())
            },
            None => {
                let ty = &field.ty;
                quote!(<#ty as reflection::Reflect>::inline_ts_definition())
            }
        };

        let field_name = field.ident.as_ref().unwrap();
        let field_name = field_name.to_string();
        let field_name = syn::Ident::new(&field_name, field_name.span());
        let field_name = match &attr.rename {
            Some(rename) => format_ident!("{}", rename),
            None => field_name
        };

        quote! {
            format!("{}: {}", stringify!(#field_name), #ts_definition)
        }
    }).collect::<TokenStream>();

    Ok(DerivedReflect {
        comment: doc,
        dependencies,
        generate_ts_definition: quote! {
            format!("{{ {} }}", #field_definitions)
        }
    })
}

fn enum_definition(e: &syn::ItemEnum, options: ReflectDeriveOptions) -> Result<DerivedReflect, Error> {
    let serde_values: Option<SerdeValues> = options.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("serde") {
            SerdeValues::from_meta(&attr.meta).ok()
        } else {
            None
        }
    });
    
    let tag = serde_values.and_then(|values| values.tag);

    Ok(DerivedReflect {
        comment: Some(format!("Tagged with {:?}.", tag)), // Temporary
        dependencies: vec![],
        generate_ts_definition: quote!(String::from("any"))
    })
}