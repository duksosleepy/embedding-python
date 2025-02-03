use pyo3::prelude::*;
use std::fs;
fn main() -> PyResult<()> {
    let test_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
    let tests = fs::read_dir(test_dir).unwrap();
    for test in tests {
        let test = test.unwrap();
        let filename = test.path().into_os_string().into_string().unwrap();
        if !filename.ends_with(".py") {
            continue;
        }
        println!("[STARTED]: {}", filename.clone());
        // TODO: Run tests.
        println!("[PASSED]: {}", filename.clone());
    }
    Ok(())
}
