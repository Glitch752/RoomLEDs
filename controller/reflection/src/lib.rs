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

    fn ts_definition() -> String;

    fn ts_type_name() -> String {
        // Remove the module path from the type name
        let type_name = std::any::type_name::<Self>();
        let type_name = type_name.split("::").last().unwrap();
        type_name.to_string()
    }
    
    fn visit_dependencies(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }

    // Exports this type and its dependencies to a file.
    fn export_all() -> Result<(), Error>
    where
        Self: 'static,
    {
        export::export::<Self>()
        // TODO: Export dependencies
    }
}