use wasm_bindgen::prelude::*;
use image_hasher::{HasherConfig, HashAlg};

/// Hashes an image and returns the hash as a base64 encoded string
#[wasm_bindgen]
pub fn hash_image(image: &[u8]) -> String {
    let hasher = HasherConfig::new().hash_alg(HashAlg::DoubleGradient).to_hasher();
    let img = image::load_from_memory_with_format(image, image::ImageFormat::Jpeg).expect("Failed to load image, image must be JPEG");

    let hash = hasher.hash_image(&img);

    hash.to_base64()
}

/**
 * Calculate the hamming distance between two hashes
 */
#[wasm_bindgen]
pub fn hamming_distance(hash1: &str, hash2: &str) -> u32 {
    let hash1: image_hasher::ImageHash<Vec<u8>> = image_hasher::ImageHash::from_base64(hash1).expect("Failed to parse hash1");
    let hash2 = image_hasher::ImageHash::from_base64(hash2).expect("Failed to parse hash2");

    hash1.dist(&hash2)
}
