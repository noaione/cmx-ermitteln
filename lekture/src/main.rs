use core::panic;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use flate2::bufread::GzDecoder;
use models::{DumpMetadata, IndexMetadata};
use serde::Serialize;
use tempfile::TempDir;

mod models;

fn read_jsonl_by_line<T>(file_path: &PathBuf) -> impl Iterator<Item = T>
where
    T: serde::de::DeserializeOwned,
{
    let file = std::fs::File::open(file_path).expect("Could not open file");
    let file = BufReader::new(file);

    file.lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            let doc: T = serde_json::from_str(&line).expect("Could not parse json");
            doc
        })
        .into_iter()
}

#[derive(Serialize)]
struct LektureDump {
    #[serde(rename = "createdAt")]
    created_at: String,
    metadata: IndexMetadata,
    count: usize,
    #[serde(rename = "documents")]
    docs: Vec<models::ErmittelnHash>,
}

fn main() {
    // get first argument
    let path = TempDir::new().expect("Could not create temp dir");
    let args = std::env::args().nth(1).expect("No argument given");

    // open file
    let dump = File::open(args).expect("Could not open file");
    let mut dump = BufReader::new(dump);
    let gz = GzDecoder::new(&mut dump);

    let mut archive = tar::Archive::new(gz);
    archive
        .unpack(path.path())
        .expect("Failed to unpack archive");

    // the index we want: path/indexes/ermitteln-images
    // which has documents.jsonl metadata.json settings.json

    let mut meta_file =
        File::open(path.path().join("metadata.json")).expect("Could not open metadata.json");
    let metadata: DumpMetadata =
        serde_json::from_reader(&mut meta_file).expect("Could not parse metadata");

    if metadata.version != "V6" {
        panic!("Only V6 dumps are supported");
    }

    println!("Dump version: {}", metadata.version);
    println!("DB version: {}", metadata.db_version);
    println!("Dump date: {}", metadata.dump_date);

    let ermitteln_index = path.path().join("indexes").join("ermitteln-images");
    if !ermitteln_index.exists() {
        panic!("`ermitteln-images` Index does not exist");
    }

    let mut index_meta_file = File::open(ermitteln_index.join("metadata.json"))
        .expect("Could not open index metadata.json");
    let index_metadata: models::IndexMetadata =
        serde_json::from_reader(&mut index_meta_file).expect("Could not parse index metadata");

    let lekture_docs: Vec<models::ErmittelnHash> =
        read_jsonl_by_line::<models::ErmittelnHash>(&ermitteln_index.join("documents.jsonl"))
            .collect();

    path.close().expect("Could not close temp dir");

    let lekture_dump = LektureDump {
        created_at: index_metadata.created_at.clone(),
        metadata: index_metadata.clone(),
        count: lekture_docs.len(),
        docs: lekture_docs,
    };

    println!("Index uid: {}", index_metadata.uid);
    println!("Index primary key: {}", index_metadata.primary_key);
    println!("Index created at: {}", index_metadata.created_at);
    println!("Index updated at: {}", index_metadata.updated_at);
    println!("Index document count: {}", lekture_dump.docs.len());

    println!("Dumping data...");
    let dump_date = metadata.dump_date;
    // strip of the time (TXXXXXX)
    let dump_date = &dump_date[..9];
    let filename = format!("lekture_{}_{}.json", index_metadata.uid, dump_date);
    println!("Writing to file: {}", filename);
    let new_file = File::create(filename);
    let mut new_file = match new_file {
        Ok(file) => file,
        Err(e) => panic!("Could not create file: {}", e),
    };

    let json = serde_json::to_string(&lekture_dump).expect("Could not serialize to json");
    match new_file.write_all(json.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(e) => panic!("Could not write to file: {}", e),
    }
}
