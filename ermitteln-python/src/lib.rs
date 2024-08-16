use pyo3::prelude::*;

/// Hashes an image and returns the hash as a base64 encoded string
#[pyfunction]
fn hash_image(image: &[u8]) -> PyResult<String> {
    let hashed_image = ::ermitteln::hash_image(image);

    match hashed_image {
        Ok(hash) => Ok(hash),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}

/// Calculate the hamming distance between two hashes
#[pyfunction]
fn hamming_distance(hash1: &str, hash2: &str) -> PyResult<u32> {
    let distance = ::ermitteln::hamming_distance(hash1, hash2);

    match distance {
        Ok(distance) => Ok(distance),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}

/// A thin-wrapper around image_hasher to make it usable in Python.
#[pymodule]
#[pyo3(name = "ermitteln")]
fn ermitteln_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_image, m)?)?;
    m.add_function(wrap_pyfunction!(hamming_distance, m)?)?;
    Ok(())
}
