use ermitteln::hamming_distance as ermitteln_hamming_distance;
use ermitteln::hash_image as ermitteln_hash_image;
use js_sys::Error;
use wasm_bindgen::prelude::*;

/// Hashes an image and returns the hash as a base64 encoded string
#[wasm_bindgen]
pub fn hash_image(image: &[u8]) -> Result<String, JsValue> {
    let hashed_image = ermitteln_hash_image(image);

    match hashed_image {
        Ok(hash) => Ok(hash),
        Err(e) => {
            let error = Error::new(&e);
            Err(error.into())
        }
    }
}

/// Calculate the hamming distance between two hashes
#[wasm_bindgen]
pub fn hamming_distance(hash1: &str, hash2: &str) -> Result<u32, JsValue> {
    let distance = ermitteln_hamming_distance(hash1, hash2);

    match distance {
        Ok(distance) => Ok(distance),
        Err(e) => {
            let error = Error::new(&e);
            Err(error.into())
        }
    }
}
