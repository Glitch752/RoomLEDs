use darling::FromMeta;
use syn::{Attribute, Expr, Field, Lit, LitStr, Meta, Path, Token};

use crate::Error;

/// Attributes, either from the reflection attribute or serde attribute, on a struct field.
#[derive(Default)]
pub struct StructFieldAttr {
    pub as_type: Option<Path>,
    pub rename: Option<String>,
    pub skip: bool,
    pub docs: Option<String>
}

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
pub struct ReflectAttr {
    pub as_type: Option<Path>
}

impl StructFieldAttr {
    pub fn from_field(field: &Field) -> Result<Self, Error> {
        let mut attr = Self::default();
        for a in field.attrs.iter() {
            if a.path().is_ident("doc") {
                let Meta::NameValue(meta) = &a.meta else { continue; };
                let Expr::Lit(value) = &meta.value else { continue; };
                let Lit::Str(doc) = &value.lit else { continue; };
            
                if attr.docs.is_none() {
                    attr.docs = Some(String::new());
                }
                attr.docs.as_mut().unwrap().push_str(&doc.value().trim());
            } else if a.path().is_ident("reflect") {
                let reflect_attr = ReflectAttr::from_meta(&a.meta)?;
                attr.as_type = reflect_attr.as_type;
            } else if a.path().is_ident("serde") {
                Self::parse_serde_attr(a, &mut attr)?;
            }
        }
        Ok(attr)
    }

    fn parse_serde_attr(attr: &syn::Attribute, field_attr: &mut StructFieldAttr) -> Result<(), Error> {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                field_attr.skip = true;
            } else if meta.path.is_ident("rename") {
                // Skip the equal sign
                meta.input.parse::<Token![=]>()?;
                let value = meta.input.parse::<LitStr>()?;
                field_attr.rename = Some(value.value());
            } else {
                // Skip to the end of the attribute
                meta.input.parse::<syn::Token![=]>()?;
                meta.input.parse::<syn::Expr>()?;
            }

            Ok(())
        }).map_err(Error::from)
    }
}