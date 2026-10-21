//! Exercise logic for the purple app.
//!
//! Pure functions over plain data: no storage, no I/O. The wasm bindings below are what the
//! Vue app calls; the same functions can be lifted into a server later.
//!
//! Nothing is modelled yet: the product requirements are still being gathered. Domain modules
//! (sessions, routines, progress) arrive with the first features, with their types exported to
//! TypeScript through tsify.

use wasm_bindgen::prelude::*;

/// The core's crate version, so the app can show which build it is running.
#[wasm_bindgen(js_name = coreVersion)]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_the_crate_version() {
        assert_eq!(core_version(), "0.1.0");
    }
}
