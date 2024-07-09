use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DownloadStat {
    downloads: i32,
    package: String,
    start : String,
    end : String
}