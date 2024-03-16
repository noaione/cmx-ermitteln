use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use rayon::prelude::*;
use serde::Serialize;
use tempfile::TempDir;

use crate::{
    common::read_jsonl_by_line,
    models::{DumpMetadata, IndexMetadata},
};

#[derive(Serialize)]
struct LektureDump {
    #[serde(rename = "createdAt")]
    created_at: String,
    metadata: IndexMetadata,
    count: usize,
    #[serde(rename = "documents")]
    docs: Vec<crate::models::ErmittelnHash>,
}

pub fn tools_redump(input: std::path::PathBuf) {
    // get first argument
    let path = TempDir::new().expect("Could not create temp dir");

    // open file
    let dump = File::open(input).expect("Could not open file");
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
    let index_metadata: crate::models::IndexMetadata =
        serde_json::from_reader(&mut index_meta_file).expect("Could not parse index metadata");

    let lekture_docs: Vec<crate::models::ErmittelnHash> =
        read_jsonl_by_line::<crate::models::ErmittelnHash>(
            &ermitteln_index.join("documents.jsonl"),
        )
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
    let dump_date = &dump_date[0..10];
    let filename = format!("lekture_{}_{}.json", index_metadata.uid, dump_date);
    println!("Writing to file: {}", filename);
    let new_file = File::create(&filename);
    let mut new_file = match new_file {
        Ok(file) => file,
        Err(e) => panic!("Could not create file: {}", e),
    };

    let json = serde_json::to_string(&lekture_dump).expect("Could not serialize to json");
    match new_file.write_all(json.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(e) => panic!("Could not write to file: {}", e),
    }

    // Package as tar gunzip
    let target = format!("lekture_{}_{}.tar.gz", index_metadata.uid, dump_date);
    let target_fp = File::create(&target).expect("Could not create target file");
    let target_buf = BufWriter::new(target_fp);
    let gz_builder = GzEncoder::new(target_buf, flate2::Compression::best());
    let mut target_tar = tar::Builder::new(gz_builder);

    let mut json_open = File::open(&filename).expect("Could not open file");
    target_tar
        .append_file(&filename, &mut json_open)
        .expect("Unable to append file");

    target_tar.finish().expect("Could not finish tarball");
}

pub fn tools_find_missing(input: std::path::PathBuf, blacklisted: Vec<String>) {
    // get first argument
    let path = TempDir::new().expect("Could not create temp dir");

    // open file
    let dump = File::open(input).expect("Could not open file");
    let mut dump = BufReader::new(dump);
    let gz = GzDecoder::new(&mut dump);

    let mut archive = tar::Archive::new(gz);
    archive
        .unpack(path.path())
        .expect("Failed to unpack archive");

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

    let mut existing_ids: Vec<usize> = vec![];
    let mut blacklisted_ids: Vec<usize> = vec![];
    read_jsonl_by_line::<crate::models::ErmittelnHash>(&ermitteln_index.join("documents.jsonl"))
        .for_each(|doc| {
            existing_ids.push(doc.id);

            if blacklisted.contains(&doc.hash) {
                blacklisted_ids.push(doc.id);
            }
        });

    println!("Total IDs: {:?}", existing_ids.len());
    println!("Blacklisted IDs: {:?}", blacklisted_ids.len());

    // Dump to file
    if !blacklisted_ids.is_empty() {
        let mut blacklisted_file =
            File::create("blacklisted_ids.txt").expect("Could not create file");
        for id in blacklisted_ids {
            writeln!(blacklisted_file, "{}", id).expect("Could not write to file");
        }
        blacklisted_file.flush().expect("Could not flush file");
        drop(blacklisted_file);
    }

    // find in-between IDs for missing
    let min_id = existing_ids.iter().min().unwrap();
    let max_id = existing_ids.iter().max().unwrap();
    // use rayon for parallelism
    let missing_ids: Vec<usize> = (*min_id..*max_id)
        .into_par_iter()
        .filter_map(|id| {
            if !existing_ids.contains(&id) {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    println!("Total Missing IDs: {:?}", missing_ids.len());

    if !missing_ids.is_empty() {
        let mut missing_file =
            File::create("missing_ids.txt").expect("Could not create file");
        for id in missing_ids {
            writeln!(missing_file, "{}", id).expect("Could not write file");
        }
        missing_file.flush().expect("Could not flush file");
        drop(missing_file);
    }

    path.close().expect("Could not close temp dir");
}
