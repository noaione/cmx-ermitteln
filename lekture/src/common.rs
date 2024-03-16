use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn read_jsonl_by_line<T>(file_path: &PathBuf) -> impl Iterator<Item = T>
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
