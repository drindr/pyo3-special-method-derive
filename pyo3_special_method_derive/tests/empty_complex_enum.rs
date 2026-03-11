use pyo3::pyclass;
use pyo3_special_method_derive_latest_pyo3::Dir;

#[pyclass]
#[derive(Dir)]
enum X {
    A {},
}
