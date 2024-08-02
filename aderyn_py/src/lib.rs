#![allow(unused)]

use aderyn_driver::driver;
use field_access::{FieldAccess, FieldMut};

fn main() {
    use pyo3::prelude::*;
    use pyo3::types::{PyBool, PyDict};

    #[pyfunction]
    #[pyo3(signature = (root, output, **py_kwargs))]
    fn generate_report(root: String, output: String, py_kwargs: Option<&Bound<'_, PyDict>>) {
        let mut args = driver::Args {
            root,
            output,
            src: None,
            no_snippets: false,
            skip_build: false,
            skip_cloc: false,
            path_includes: None,
            path_excludes: None,
            stdout: false,
            skip_update_check: false,
            auditor_mode: false,
            highs_only: false,
        };

        if let Some(kwargs) = py_kwargs {
            kwargs.iter().for_each(|(py_key, py_value)| {
                let rust_key: String = py_key.extract().unwrap();
                if py_value.is_instance_of::<PyBool>() {
                    let rust_value: bool = py_value.extract().unwrap();
                    args.field_mut(&rust_key).unwrap().replace(rust_value);
                } else {
                    let rust_value: Vec<String> = py_value.extract().unwrap();
                    args.field_mut(&rust_key).unwrap().replace(Some(rust_value));
                }
            })
        }

        driver::drive(args);
    }

    /// A Python module implemented in Rust. The name of this function must match
    /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
    /// import the module.
    #[pymodule]
    fn aderynpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(generate_report, m)?)?;

        Ok(())
    }
}
