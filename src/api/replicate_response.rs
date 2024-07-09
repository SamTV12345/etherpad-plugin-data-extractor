use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReplicateResponse {
    pub results: Vec<ReplicateResult>
}

#[derive(Serialize, Deserialize)]
pub struct ReplicateResult {
    pub  seq: i32,
    pub  id: String,
    pub  changes: Vec<Change>,
    pub  doc:  ReplicateResultDoc
}

#[derive(Serialize, Deserialize)]
pub struct ReplicateResultDoc {
    pub  _id: String,
    pub  _rev: String,
    pub  name: String,
    pub  description: Option<String>,
    #[serde(rename = "dist-tags")]
    pub  dist_tags: DistTags,
    pub  readme: Option<String>,
    pub  maintainers: Option<Vec<Maintainer>>,
    pub  versions: HashMap<String, ReplicateVersion>,
    pub  time: ReplicateTime
}

#[derive(Serialize, Deserialize)]
pub struct ReplicateVersion {
    pub name: String,
    pub description: Option<String>,
    pub deprecated: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Maintainer {
    pub  name: String,
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct DistTags {
    pub latest: String
}

#[derive(Serialize, Deserialize)]
pub struct Change {
    pub rev: String
}

#[derive(Serialize, Deserialize)]
pub struct ReplicateTime {
    pub created: String,
    pub modified: String
}