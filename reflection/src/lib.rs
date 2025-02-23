/// A simple custom reflection system.

extern crate proc_macro;
use proc_macro2::TokenStream;

trait Reflect {
    
}

/// A derive macro that generates a test to create reflection bindings for the specified type.
#[proc_macro_derive(Reflect, attributes(serde, reflect))]
pub fn derive_reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO
    proc_macro::TokenStream::new()
}