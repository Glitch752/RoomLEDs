
use crate::{Error, Reflect, TypeVisitor};

use super::{export, export_schema};

/// Exports T and its dependencies.
pub(crate) fn export_recursively<T: Reflect + ?Sized + 'static>() -> Result<(), Error> {
    if !T::INLINE {
        let exported = export::<T>()?;
        if !exported {
            return Ok(());
        }
    }

    let mut visitor = ExportVisitor {
        error: None
    };
    T::visit_dependencies(&mut visitor);

    if let Some(e) = visitor.error {
        Err(e)
    } else {
        Ok(())
    }
}

// Exports the schema for T and its dependencies.
pub(crate) fn export_schema_recursively<T: Reflect + ?Sized + 'static>() -> Result<(), Error> {
    if !T::INLINE {
        let exported = export_schema::<T>()?;
        if !exported {
            return Ok(());
        }
    }

    let mut visitor = SchemaExportVisitor {
        error: None
    };
    T::visit_dependencies(&mut visitor);

    if let Some(e) = visitor.error {
        Err(e)
    } else {
        Ok(())
    }
}

struct ExportVisitor {
    error: Option<Error>
}

impl TypeVisitor for ExportVisitor {
    fn visit_export<T: Reflect + 'static + ?Sized>(&mut self) {
        // if an error occurred previously or the type can't be exported, return early
        if self.error.is_some() {
            return;
        }

        self.error = export_recursively::<T>().err();
    }
}

struct SchemaExportVisitor {
    error: Option<Error>
}

impl TypeVisitor for SchemaExportVisitor {
    fn visit_export<T: Reflect + 'static + ?Sized>(&mut self) {
        // if an error occurred previously or the type can't be exported, return early
        if self.error.is_some() {
            return;
        }

        self.error = export_schema_recursively::<T>().err();
    }
}