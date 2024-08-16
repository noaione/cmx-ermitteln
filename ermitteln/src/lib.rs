use image_hasher::{HashAlg, Hasher, HasherConfig};

/// Create a new instance of Hasher
///
/// Our hasher uses the [`HashAlg::DoubleGradient`] algorithm and [`HasherConfig::preproc_dct()`] preprocessor
/// to combine pHash (DCT) with dHash (with double grad)
fn create_hasher() -> Hasher {
    HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .preproc_dct()
        .to_hasher()
}

pub fn hash_image(image: &[u8]) -> Result<String, String> {
    let hasher = create_hasher();
    let img = image::load_from_memory_with_format(image, image::ImageFormat::Jpeg);

    match img {
        Ok(img) => {
            let hash = hasher.hash_image(&img);
            Ok(hash.to_base64())
        }
        Err(e) => {
            let error = format!("Failed to load image: {}", e);
            Err(error)
        }
    }
}

/// Calculate the hamming distance between two hashes
pub fn hamming_distance(hash1: &str, hash2: &str) -> Result<u32, String> {
    let hash1 = image_hasher::ImageHash::<Vec<u8>>::from_base64(hash1);
    let hash2 = image_hasher::ImageHash::<Vec<u8>>::from_base64(hash2);

    match (hash1, hash2) {
        (Ok(hash1), Ok(hash2)) => Ok(hash1.dist(&hash2)),
        (Err(e), _) | (_, Err(e)) => {
            let error = format!("Failed to load hash: {:?}", e);
            Err(error)
        }
    }
}
