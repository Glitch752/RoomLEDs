/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

mod export;
pub mod schema;

use std::{rc::Rc, sync::Arc};

pub use reflection_derive::Reflect;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("The environment variable CARGO_MANIFEST_DIR must be set.")]
    ManifestDirNotSet
}

pub trait TypeVisitor: Sized {
    fn visit_export<T: Reflect + 'static + ?Sized>(&mut self);
}

pub trait Reflect {
    const JSDOC_COMMENT: Option<&'static str> = None;
    const INLINE: bool = false;

    fn ts_definition() -> String;

    fn noninline_ts_definition() -> String {
        if Self::INLINE {
            Self::ts_definition()
        } else {
            Self::ts_type_name()
        }
    }

    fn schema() -> schema::Schema;

    fn schema_reference() -> schema::Schema {
        schema::Schema::Reference(Self::ts_type_name())
    }

    fn ts_type_name() -> String {
        // Remove the module path from the type name
        let type_name = std::any::type_name::<Self>();
        let type_name = type_name.split("::").last().unwrap();
        // Remove the last ">" from the type name if there is one
        let type_name = type_name.trim_end_matches('>');
        type_name.to_string()
    }
    
    /// Visits the dependencies of this type.
    fn visit_dependencies(_: &mut impl TypeVisitor) where Self: 'static;

    /// Exports this type and its dependencies to a file.
    fn export_all() -> Result<(), Error> where Self: 'static {
        export::export_recursively::<Self>()
    }

    /// Exports the schema for this type and its dependencies to a file.
    fn export_all_schema() -> Result<(), Error> where Self: 'static {
        export::export_schema_recursively::<Self>()
    }
}

// Implement Reflect for common types
impl <T: Reflect> Reflect for Option<T> {
    const INLINE: bool = true;

    fn ts_definition() -> String {
        format!("{} | null", T::noninline_ts_definition())
    }

    fn schema() -> schema::Schema {
        schema::Schema::Optional(Box::new(T::schema()))
    }

    fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
        visitor.visit_export::<T>();
    }
}

impl <T: Reflect> Reflect for Vec<T> {
    const INLINE: bool = true;

    fn ts_definition() -> String {
        format!("Array<{}>", T::noninline_ts_definition())
    }

    fn schema() -> schema::Schema {
        schema::Schema::ArrayOf(Box::new(T::schema()))
    }

    fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
        visitor.visit_export::<T>();
    }
}

macro_rules! container_reflect {
    ($($ty:ident),*) => {
        $(
            impl<T: Reflect> Reflect for $ty<T> {
                const INLINE: bool = true;

                fn ts_definition() -> String {
                    T::ts_definition()
                }
                fn noninline_ts_definition() -> String {
                    T::noninline_ts_definition()
                }
                
                fn schema() -> schema::Schema {
                    T::schema()
                }
                fn schema_reference() -> schema::Schema {
                    T::schema_reference()
                }
            
                fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
                    visitor.visit_export::<T>();
                }
            }
        )*
    };
}

container_reflect!(Box, Rc, Arc);

macro_rules! basic_reflect {
    ($($ty:ty, $def:expr, $schema:ident);*) => {
        $(
            impl Reflect for $ty {
                const INLINE: bool = true;
                fn ts_definition() -> String {
                    $def.to_string()
                }

                fn schema() -> schema::Schema {
                    schema::Schema::$schema
                }
                fn schema_reference() -> schema::Schema {
                    schema::Schema::$schema
                }

                fn visit_dependencies(_: &mut impl TypeVisitor) {}
            }
        )*
    };
}

basic_reflect! {
    bool, "boolean", Boolean;
    i8, "number", Number;
    i16, "number", Number;
    i32, "number", Number;
    i64, "number", Number;
    u8, "number", Number;
    u16, "number", Number;
    u32, "number", Number;
    u64, "number", Number;
    f32, "number", Number;
    f64, "number", Number;
    char, "string", String;
    String, "string", String;
    &str, "string", String
}

macro_rules! tuple_reflect {
    ($($ty:ident),*) => {
        impl<$($ty: Reflect),*> Reflect for ($($ty,)*) {
            const INLINE: bool = true;
            fn ts_definition() -> String {
                format!("[{}]", vec![$($ty::noninline_ts_definition()),*].join(", "))
            }

            fn schema() -> schema::Schema {
                schema::Schema::TupleOf(vec![$($ty::schema()),*])
            }

            fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
                $(visitor.visit_export::<$ty>();)*
            }
        }
    };
}

tuple_reflect!(A);
tuple_reflect!(A, B);
tuple_reflect!(A, B, C);
tuple_reflect!(A, B, C, D);
tuple_reflect!(A, B, C, D, E);