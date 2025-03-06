
use crate::{Error, Reflect, TypeVisitor};

use super::export;

/// Exports T and its dependencies.
pub(crate) fn export_recursively<T: Reflect + ?Sized + 'static>() -> Result<(), Error> {
    export::<T>()?;

    let mut visitor = Visitor {
        error: None
    };
    T::visit_dependencies(&mut visitor);

    if let Some(e) = visitor.error {
        Err(e)
    } else {
        Ok(())
    }
}

struct Visitor {
    error: Option<Error>
}

impl TypeVisitor for Visitor {
    fn visit<T: Reflect + 'static + ?Sized>(&mut self) {
        // if an error occurred previously or the type can't be exported, return early
        if self.error.is_some() {
            return;
        }

        // TODO: Inline trivial types like i32, f32, String, etc.

        self.error = export_recursively::<T>().err();
    }
}