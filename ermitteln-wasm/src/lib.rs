use image_hasher::{HashAlg, HasherConfig};
use js_sys::Error;
use wasm_bindgen::prelude::*;

/// Hashes an image and returns the hash as a base64 encoded string
#[wasm_bindgen]
pub fn hash_image(image: &[u8]) -> Result<String, JsValue> {
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .to_hasher();
    let img = image::load_from_memory_with_format(image, image::ImageFormat::Jpeg);

    match img {
        Ok(img) => Ok(hasher.hash_image(&img).to_base64()),
        Err(e) => {
            let error = Error::new(&format!("Failed to load image: {}", e));
            Err(error.into())
        }
    }
}

/**
 * Calculate the hamming distance between two hashes
 */
#[wasm_bindgen]
pub fn hamming_distance(hash1: &str, hash2: &str) -> Result<u32, JsValue> {
    let hash1 = image_hasher::ImageHash::<Vec<u8>>::from_base64(hash1);
    let hash2 = image_hasher::ImageHash::<Vec<u8>>::from_base64(hash2);

    match (hash1, hash2) {
        (Ok(hash1), Ok(hash2)) => Ok(hash1.dist(&hash2)),
        (Err(e), _) | (_, Err(e)) => {
            let error = Error::new(&format!("Failed to load hash: {:?}", e));
            Err(error.into())
        }
    }
}
