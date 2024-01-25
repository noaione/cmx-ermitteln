// dump v6 models, meilisearch 1.6+

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct IndexMetadata {
    pub(crate) uid: String,
    #[serde(rename = "primaryKey")]
    pub(crate) primary_key: String,
    #[serde(rename = "createdAt")]
    pub(crate) created_at: String,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DumpMetadata {
    #[serde(rename = "dumpVersion")]
    pub(crate) version: String,
    #[serde(rename = "dbVersion")]
    pub(crate) db_version: String,
    #[serde(rename = "dumpDate")]
    pub(crate) dump_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ErmittelnHash {
    pub(crate) id: usize,
    pub(crate) hash: String,
}
