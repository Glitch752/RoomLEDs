/// A custom reflection system to access type data from Rust in Typescript.
/// Heavily based on [ts-rs](https://github.com/Aleph-Alpha/ts-rs/); this is essentially just a stripped-down version of it. 

mod export;

pub use reflection_derive::Reflect;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("The environment variable CARGO_MANIFEST_DIR must be set.")]
    ManifestDirNotSet
}

pub trait TypeVisitor: Sized {
    fn visit<T: Reflect + 'static + ?Sized>(&mut self);
}

pub trait Reflect {
    const JSDOC_COMMENT: Option<&'static str> = None;
    const INLINE: bool = false;

    fn ts_definition() -> String;

    fn inline_ts_definition() -> String {
        if Self::INLINE {
            Self::ts_definition()
        } else {
            Self::ts_type_name()
        }
    }

    fn ts_type_name() -> String {
        // Remove the module path from the type name
        let type_name = std::any::type_name::<Self>();
        let type_name = type_name.split("::").last().unwrap();
        type_name.to_string()
    }
    
    /// Visits the dependencies of this type.
    fn visit_dependencies(_: &mut impl TypeVisitor) where Self: 'static;

    // Exports this type and its dependencies to a file.
    fn export_all() -> Result<(), Error> where Self: 'static {
        export::export_recursively::<Self>()
    }
}

// Implement Reflect for common types
impl <T: Reflect> Reflect for Option<T> {
    const INLINE: bool = true;

    fn ts_definition() -> String {
        format!("{} | null", T::ts_definition())
    }

    fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
        T::visit_dependencies(visitor);
    }
}

impl <T: Reflect> Reflect for Vec<T> {
    const INLINE: bool = true;

    fn ts_definition() -> String {
        format!("Array<{}>", T::ts_definition())
    }

    fn visit_dependencies(visitor: &mut impl TypeVisitor) where Self: 'static {
        T::visit_dependencies(visitor);
    }
}

macro_rules! basic_reflect {
    ($($ty:ty, $def:expr);*) => {
        $(
            impl Reflect for $ty {
                const INLINE: bool = true;
                fn ts_definition() -> String {
                    $def.to_string()
                }

                fn visit_dependencies(_: &mut impl TypeVisitor) {}
            }
        )*
    };
}

basic_reflect! {
    bool, "boolean";
    i8, "number";
    i16, "number";
    i32, "number";
    i64, "number";
    u8, "number";
    u16, "number";
    u32, "number";
    u64, "number";
    f32, "number";
    f64, "number";
    char, "string";
    String, "string"
}