
use pyo3::prelude::*;

mod rbo;
// use crate::rbo;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn rank_biased_overlap(
    search_depth: usize, p: f64, 
    left: Vec<String>, 
    right: Vec<String>) -> PyResult<f64> {
    let conf = rbo::Config::new(Some(search_depth),p);
    let left_ranked = rbo::RankedList::new(left).unwrap();
    let right_ranked = rbo::RankedList::new(right).unwrap(); 
    Ok(rbo::rbo(conf,left_ranked,right_ranked))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fastrbo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rank_biased_overlap, m)?)?;
    Ok(())
}