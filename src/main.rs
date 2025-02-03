use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let test_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
    let tests = fs::read_dir(test_dir).unwrap();
    for test in tests {
        let test = test.unwrap();
        let filename = test.path().into_os_string().into_string().unwrap();
        if !filename.ends_with(".py") {
            continue;
        }
        println!("[STARTED]: {}", filename.clone());
        let module_name = test
            .path()
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let source = fs::read_to_string(filename.clone())?;

        Python::with_gil(|py| -> PyResult<()> {
            PyModule::from_code(
                py,
                &CString::new(source.as_str())?.as_c_str(),
                &CString::new(filename.as_str())?.as_c_str(),
                &CString::new(module_name.as_str())?.as_c_str(),
            )?;
            Ok(())
        })?;
        println!("[PASSED]: {}", filename.clone());
    }
    Ok(())
}
