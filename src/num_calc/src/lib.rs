use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
fn get_stats(inp: f32) -> HashMap<String, f32> {
    HashMap::from([
        ("in_half".to_string(), inp / 2.0),
        ("doubled".to_string(), inp * 2.0),
        ("squared".to_string(), inp * inp),
        ("cosine".to_string(), inp.cos()),
        ("sin".to_string(), inp.sin()),
        ("tangent".to_string(), inp.tan()),
    ])
}

#[pymodule]
fn num_calc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_stats, m)?)?;

    Ok(())
}
