use pyo3::pyclass;
use pyo3_special_method_derive_latest_pyo3::Dir;

#[derive(PartialEq)]
#[pyclass(eq, eq_int)]
#[derive(Dir)]
#[allow(dead_code)]
enum Tester {
    Alpha,
    #[skip(Dir)]
    Beta,
}

#[test]
fn test_with_dir() {
    let dir = Tester::Beta.__dir__();

    assert!(dir.is_empty());
}
