/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

extern crate proc_macro;

use std::rc::Rc;

use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Item, Meta, Path, Type};

mod fields;

struct DerivedReflect {
    pub comment: Option<String>,
    pub dependencies: Vec<Rc<Path>>,
    pub generate_ts_definition: TokenStream,
    pub generate_schema: TokenStream
}

impl DerivedReflect {
    fn implementation(self, ident: syn::Ident, export_runtime_schema: bool) -> TokenStream {
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
            quote! { visitor.visit_export::<#ty>(); }
        }).collect::<TokenStream>();

        let generate_ts_definition = self.generate_ts_definition;
        let generate_schema = self.generate_schema;

        let schema_test = if export_runtime_schema {
            let schema_test_name = format_ident!("export_schema_{}", ty.to_string().to_lowercase().replace("r#", ""));
            Some(quote! {
                #[cfg(test)]
                #[test]
                fn #schema_test_name() {
                    #ty::export_all_schema().expect("could not export schema");
                }
            })
        } else { None };

        quote! {
            impl reflection::Reflect for #ty {
                const JSDOC_COMMENT: Option<&'static str> = #comment_option;

                fn ts_definition() -> String {
                    #generate_ts_definition
                }

                fn schema() -> reflection::schema::Schema {
                    #generate_schema
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

            #schema_test
        }
    }
}

/// Options for the derive macro.
#[derive(FromDeriveInput, Default, Debug)]
#[darling(default, attributes(reflect), forward_attrs(doc, serde))]
struct ReflectDeriveOptions {
    attrs: Vec<syn::Attribute>,
    /// Whether to recursively export the schema to a value so it can be used in runtime.
    export_runtime_schema: bool
}

impl ReflectDeriveOptions {
    fn get_doc(&self) -> String {
        self.attrs.iter().filter_map(|attr| {
            if attr.path().is_ident("doc") {
                match &attr.meta {
                    Meta::NameValue(meta) => {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value {
                            let value = lit.value();
                            Some(value.trim().to_string())
                        } else {
                            None
                        }
                    },
                    _ => None
                }
                // attr.meta.parse_args::<syn::LitStr>().ok().map(|lit| lit.value())
            } else {
                None
            }
        }).collect::<Vec<_>>().join("\n")
    }
}

// Values from serde's attribute we care about
#[derive(FromMeta, Debug)]
struct SerdeValues {
    tag: Option<String>,
    content: Option<String>
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

    let export_runtime_schema = options.export_runtime_schema;

    let (reflect, identifier) = match &ast {
        Item::Struct(s) => Ok((struct_definition(s, options)?, s.ident.clone())),
        Item::Enum(e) => Ok((enum_definition(e, options)?, e.ident.clone())),
        _ => Err(Error::UnsupportedUse("only structs and enums are supported")),
    }?;

    Ok(reflect.implementation(identifier, export_runtime_schema))
}

fn struct_definition(s: &syn::ItemStruct, options: ReflectDeriveOptions) -> Result<DerivedReflect, Error> {
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
                quote!(<#ty as reflection::Reflect>::noninline_ts_definition())
            },
            None => {
                let ty = &field.ty;
                quote!(<#ty as reflection::Reflect>::noninline_ts_definition())
            }
        };

        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_name = syn::Ident::new(&field_name, field_name.span());
        let field_name = match &attr.rename {
            Some(rename) => format_ident!("{}", rename),
            None => field_name
        };

        let jsdoc_comment = match &attr.docs {
            Some(comment) => format!("\n/**\n{}\n */\n", comment.lines()
                .map(|line| format!(" * {}", line))
                .collect::<Vec<_>>().join("\n")
            ),
            None => String::new()
        };

        quote! {
            format!("{}{}: {}", #jsdoc_comment, stringify!(#field_name), #ts_definition)
        }
    }).collect::<Vec<_>>();

    let schema_fields = fields.iter().map(|(field, attr)| {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_name = syn::Ident::new(&field_name, field_name.span());
        let field_name = match &attr.rename {
            Some(rename) => format_ident!("{}", rename),
            None => field_name
        };

        let schema_reference = match &attr.as_type {
            Some(ty) => {
                quote!(<#ty as reflection::Reflect>::schema_reference())
            },
            None => {
                let ty = &field.ty;
                quote!(<#ty as reflection::Reflect>::schema_reference())
            }
        };

        let docs = match &attr.docs {
            Some(v) => {
                quote!(Some(#v.to_string()))
            },
            None => {
                quote!(None)
            }
        };

        quote! {
            reflection::schema::SchemaField {
                name: stringify!(#field_name).to_string(),
                ty: #schema_reference,
                docs: #docs
            }
        }
    }).collect::<Vec<_>>();
    
    Ok(DerivedReflect {
        comment: Some(options.get_doc()),
        dependencies,
        generate_ts_definition: quote! {
            format!("{{ {} }}", <[String]>::join(&[ #(#field_definitions),* ], ", "))
        },
        generate_schema: quote! {
            reflection::schema::Schema::Struct(vec![
                #(#schema_fields),*
            ])
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
    
    let tag = serde_values.as_ref().and_then(|values| values.tag.clone());
    let content = serde_values.and_then(|values| values.content);

    if tag.is_none() {
        return Err(Error::UnsupportedUse("Tagged enums are required for now."));
    }
    let tag = tag.unwrap();

    let dependencies = e.variants.iter().map(|variant| {
        match &variant.fields {
            syn::Fields::Named(_) => {
                return Err(Error::UnsupportedUse("named fields are not supported"));
            },
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().filter_map(|field| {
                let ty = &field.ty;
                if let Type::Path(path) = ty {
                    Some(Ok(Rc::new(path.path.clone())))
                } else {
                    None
                }
            }).collect::<Result<Vec<_>, Error>>(),
            syn::Fields::Unit => Ok(vec![])
        }
    }).collect::<Result<Vec<_>, Error>>()?.into_iter().flatten().collect::<Vec<_>>();

    let variants = e.variants.iter().map(|variant| {
        let variant_name = variant.ident.to_string();

        Ok(match &variant.fields {
            syn::Fields::Named(_) => {
                return Err(Error::UnsupportedUse("named fields are not supported"));
            },
            syn::Fields::Unnamed(fields) => {
                fields.unnamed.iter().map(|field| {
                    let ts_definition = match &field.ty {
                        Type::Path(path) => {
                            quote!(<#path as reflection::Reflect>::noninline_ts_definition())
                        },
                        _ => quote!("unknown")
                    };

                    if let Some(content) = &content {
                        quote!(format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #variant_name, #content, #ts_definition))
                    } else {
                        quote!(format!("{{ \"{}\": \"{}\" }} & {}", #tag, #variant_name, #ts_definition))
                    }
                }).collect::<TokenStream>()
            },
            syn::Fields::Unit => {
                quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #variant_name))
            }
        })
    }).collect::<Result<Vec<_>, Error>>()?;

    let schema_variants = e.variants.iter().map(|variant| {
        let variant_name = variant.ident.to_string();
        let schema = match &variant.fields {
            syn::Fields::Named(_) => {
                return Err(Error::UnsupportedUse("named fields are not supported"));
            },
            syn::Fields::Unnamed(fields) => {
                fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;
                    quote!(Some(<#ty as reflection::Reflect>::schema_reference()))
                }).collect::<TokenStream>()
            },
            syn::Fields::Unit => {
                quote!(None)
            }
        };

        Ok(quote! {
            reflection::schema::EnumVariant {
                name: String::from(#variant_name),
                value: #schema
            }
        })
    }).collect::<Result<Vec<_>, Error>>()?;

    let content_tokens = match &content {
        Some(content) => quote!(Some(String::from(#content))),
        None => quote!(None)
    };

    Ok(DerivedReflect {
        comment: Some(format!("Tagged with {:?}.\n{}", tag, options.get_doc())), // Temporary?
        dependencies,
        generate_ts_definition: quote! (
            [ #(#variants),* ].join(" | ")
        ),
        generate_schema: quote! {
            reflection::schema::Schema::Enum(reflection::schema::EnumValue {
                variants: vec![
                    #(#schema_variants),*
                ],
                tag_name: String::from(#tag),
                content_subfield: #content_tokens
            })
        }
    })
}