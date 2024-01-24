use std::path::PathBuf;

use clap::Parser;
use dotenv::dotenv;
use image_hasher::{HashAlg, HasherConfig};
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::cli::{AufnehmenCli, AufnehmenCommands};

mod cli;

const BLACKLISTED_HASH: [&str; 11] = [
    "f91cfdf3fcbfd1012c04c54b00cde53590d0936c39889964a5a452518de77d57", // Black CMX Placeholder
    "9b03356e437cabbab48270a7c3e811306cf147c589a85be101e16da8cef33275", // Black CMX Placeholder (variant)
    "dc4b594e62e54ab824c28d2686fae8a8b0397f9bbd04de4d452b9b69452ca21e", // White CMX Placeholder
    "6a2055b188a0983162e5810266669acb0b351361b33fe3d1a65b63c3ef2d3a3f", // White CMX Placeholder (variant)
    "d7eb535c6af1ed945799ce44ac51f3b2af5bd9cf94594e6609b834021ded2fdb", // White CMX Placeholder (variant square)
    "08c6433b2c9ad47a36c90a1b9470b0ab8ec614e31ec518d6f4e4c2383eaa8e54", // White CMX Placeholder (variant; minimal)
    "b7c5349d51aadf28a885de88bad7cc44f901f6401519f728c828700f4564d135", // DC Placeholder
    "5e7e7b0b37ba5c74b49f78ab913bd1b9757e68dfe73a2a3f7f27438b331df05d", // DC Placeholder (variant)
    "bf20292f29fcf8138c9ba2340921e6cd39732c71aaaa796dccf7ae0a3461ae76", // IDW Placeholder
    "77bb266e0abb6dbdb85fdd709ba92619c003ab1c2030ca699813c51dc0123c07", // Beaver thing
    "8e1ac854a5233fef5b2e7cb28219cc37f1daf4cf5628e70be7627de133c60e76", // Marvel Placeholder
];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErmittelnHash {
    id: usize,
    hash: String,
}

pub fn hash_image(image: &[u8]) -> (String, String) {
    let sha2_hash = <Sha256 as Digest>::digest(image);
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .to_hasher();
    let img = image::load_from_memory_with_format(image, image::ImageFormat::Jpeg)
        .expect("Failed to load image, image must be JPEG");

    let hash = hasher.hash_image(&img);

    (hash.to_base64(), format!("{:x}", sha2_hash))
}

async fn ingest_handler(
    input_folder: PathBuf,
    chunk_size: usize,
    start_id: usize,
    end_id: Option<usize>,
    meili_url: String,
    meili_key: String,
    verbose: bool,
) {
    println!("Starting aufnehmen indexer...");
    // get all files in folder
    let files = std::fs::read_dir(input_folder).expect("Unable to read folder.");
    let mut jpeg_images = files
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();

            // check extension (jpg/jpeg only)
            let extension = path.extension()?;
            if extension == "jpg" || extension == "jpeg" {
                let id_num = path.file_stem()?.to_str()?.parse::<usize>().ok()?;
                // if id_num is less than start_id, skip
                if id_num < start_id {
                    return None;
                }
                // if end_id is set, and id_num is greater than end_id, skip
                if let Some(end_id) = end_id {
                    if id_num > end_id {
                        return None;
                    }
                }
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // ascending sort
    jpeg_images.sort_by(|a, b| {
        let a = a
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let b = b
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        a.cmp(&b)
    });
    // reverse to descending
    jpeg_images.reverse();

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
    let chunks = jpeg_images.chunks(chunk_size);
    let total_chunk = chunks.len();
    for (idx, chunk) in chunks.enumerate() {
        // hash images
        let mut hashes = vec![];
        println!(
            "  Hashing {} images... ({}/{} chunks)",
            chunk.len(),
            idx + 1,
            total_chunk
        );
        for image_path in chunk {
            let filename = image_path.file_stem().unwrap().to_str().unwrap();
            if verbose {
                println!("    Reading image: {}", filename);
            }
            let read_image = tokio::fs::read(image_path)
                .await
                .unwrap_or_else(|_| panic!("Failed to read image: {}", filename));
            if verbose {
                println!("     Hashing image: {}", filename);
            }
            let hash = tokio::task::spawn_blocking(move || hash_image(&read_image)).await;
            let (hash, sha2_hash) =
                hash.unwrap_or_else(|_| panic!("Failed to hash image: {}", filename));

            if BLACKLISTED_HASH.contains(&&*sha2_hash) {
                println!("    Skipping blacklisted image: {}", filename);
                continue;
            }

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

#[tokio::main]
async fn main() {
    dotenv().ok();

    // For some god know what reason, `clap` + rustc_lint will show this as unreachable code.
    let _cli = AufnehmenCli::parse();

    match _cli.command {
        AufnehmenCommands::Hash { input } => {
            let image = tokio::fs::read(input).await.expect("Failed to read image.");
            let hash = tokio::task::spawn_blocking(move || hash_image(&image)).await;
            let (hash, b64_hash) = hash.expect("Failed to hash image.");
            println!("Hash: {}", hash);
            println!("SHA256 Hash: {}", b64_hash);
        }
        AufnehmenCommands::Ingest {
            input,
            chunk_size,
            start_id,
            end_id,
        } => {
            if chunk_size > 1000 {
                println!("Chunk size cannot be greater than 1000!");
                std::process::exit(1);
            }

            let meili_url = match std::env::var("MEILI_URL") {
                Ok(url) => url,
                Err(_) => {
                    println!("MEILI_URL env is not set!");
                    std::process::exit(1);
                }
            };
            let meili_key = match std::env::var("MEILI_KEY") {
                Ok(key) => key,
                Err(_) => {
                    println!("MEILI_KEY env is not set!");
                    std::process::exit(1);
                }
            };

            ingest_handler(
                input,
                chunk_size,
                start_id,
                end_id,
                meili_url,
                meili_key,
                _cli.verbose,
            )
            .await;
        }
    };
}
