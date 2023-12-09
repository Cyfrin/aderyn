#![allow(unused)]

use aderyn_driver::driver;

fn main() {
    use pyo3::prelude::*;

    #[pyfunction]
    fn generate_report(root: String, output: String) {
        let args = driver::Args { root, output };
        driver::drive(args);
    }

    /// A Python module implemented in Rust. The name of this function must match
    /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
    /// import the module.
    #[pymodule]
    fn aderynpy(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(generate_report, m)?)?;

        Ok(())
    }
}
