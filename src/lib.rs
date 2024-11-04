use markdown::{to_mdast, ParseOptions};
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyModuleMethods, PyResult};
use pyo3::{wrap_pyfunction, Bound};
use pyo3::exceptions::PyRuntimeError;
use mdast_util_to_markdown::{
    to_markdown
};

#[pyfunction]
/// Convert markdown to an mdast json representation.
/// Returns json because that's way easier than building a python object from rust
fn md_to_json(md: &str) -> PyResult<String> {
    let tree = to_mdast(md, &ParseOptions::default())
        .map_err(|_| PyRuntimeError::new_err("Failed to parse Markdown"))?;
    let json = serde_json::to_string(&tree)
        .map_err(|_| PyRuntimeError::new_err("Failed to convert to JSON"))?;
    Ok(json)
}

#[pyfunction]
/// Convert mdast json back to markdown.
fn json_to_md(json: &str) -> PyResult<String> {
    let tree: markdown::mdast::Node = serde_json::from_str(json)
        .map_err(|_| PyRuntimeError::new_err("Failed to parse json back to node"))?;

    let md = to_markdown(&tree)
        .map_err(|_| PyRuntimeError::new_err("Failed to convert node back to markdown"))?;

    Ok(md)
}

#[pymodule]
fn mdast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(md_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(json_to_md, m)?)?;
    Ok(())
}
