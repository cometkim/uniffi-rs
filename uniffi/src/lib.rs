/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Reexport items from other uniffi creates
pub use uniffi_core::*;
pub use uniffi_macros::{export, include_scaffolding, Enum, Error, Object, Record};
#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "bindgen-tests")]
pub use uniffi_bindgen::bindings::kotlin::run_test as kotlin_run_test;
#[cfg(feature = "bindgen-tests")]
pub use uniffi_bindgen::bindings::python::run_test as python_run_test;
#[cfg(feature = "bindgen-tests")]
pub use uniffi_bindgen::bindings::ruby::run_test as ruby_run_test;
#[cfg(feature = "bindgen-tests")]
pub use uniffi_bindgen::bindings::swift::run_test as swift_run_test;
#[cfg(feature = "bindgen")]
pub use uniffi_bindgen::{generate_bindings, generate_component_scaffolding, print_json};
#[cfg(feature = "build")]
pub use uniffi_build::generate_scaffolding;
#[cfg(feature = "bindgen-tests")]
pub use uniffi_macros::build_foreign_language_testcases;

#[cfg(test)]
mod test {
    #[test]
    fn trybuild_ui_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}

#[cfg(feature = "cli")]
pub fn uniffi_bindgen_main() {
    cli::run_main().unwrap();
}
