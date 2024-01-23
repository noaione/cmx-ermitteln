use dotenv::dotenv;
use image_hasher::{HashAlg, HasherConfig};
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErmittelnHash {
    id: usize,
    hash: String,
}

pub fn hash_image(image: &[u8]) -> String {
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .to_hasher();
    let img = image::load_from_memory_with_format(image, image::ImageFormat::Jpeg)
        .expect("Failed to load image, image must be JPEG");

    let hash = hasher.hash_image(&img);

    hash.to_base64()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let meili_url = std::env::var("MEILI_URL").expect("MEILI_URL is not set.");
    let meili_key = std::env::var("MEILI_KEY").expect("MEILI_KEY is not set.");

    // get first argument for a folder
    let folder = std::env::args().nth(1).expect("No folder given.");

    println!("Starting aufnehmen indexer...");
    // get all files in folder
    let files = std::fs::read_dir(folder).expect("Unable to read folder.");
    let jpeg_images = files
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();
            let extension = path.extension()?;
            if extension == "jpg" || extension == "jpeg" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!(
        "Found {} images, connecting to Meilisearch...",
        jpeg_images.len()
    );

    // create a client
    let client = Client::new(&meili_url, Some(&meili_key));
    let version = client.get_version().await.unwrap();
    println!("Connected to Meilisearch version {:?}", version);

    let ermitteln_index = client.index("ermitteln-images");

    // chunk the images into 1000 images per chunk
    println!("Hashing a total of {} images...", jpeg_images.len());
    let chunks = jpeg_images.chunks(1000);
    for chunk in chunks {
        // hash images
        let mut hashes = vec![];
        println!("  Hashing {} images...", chunk.len());
        for image_path in chunk {
            let filename = image_path.file_stem().unwrap().to_str().unwrap();
            let read_image = tokio::fs::read(image_path)
                .await
                .expect("Failed to read image.");

            let hash = tokio::task::spawn_blocking(move || hash_image(&read_image)).await;
            let hash = hash.expect("Failed to hash image.");

            hashes.push(ErmittelnHash {
                id: filename.parse().unwrap(),
                hash,
            })
        }

        hashes.sort_by(|a, b| a.id.cmp(&b.id));
        // first ID
        let first_id = hashes.clone().first().unwrap().id;
        let last_id = hashes.clone().last().unwrap().id;

        // send hashes to meilisearch
        println!(
            "   Sending {}-{} hashes to meilisearch...",
            first_id, last_id
        );
        let task = ermitteln_index
            .add_or_replace(&hashes, Some("id"))
            .await
            .unwrap();
        println!("    Waiting for indexing to finish...");
        let _ = task.wait_for_completion(&client, None, None).await;
    }
}
