use std::collections::{HashMap, HashSet};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

pub fn download_current_plugins(){
    let res = reqwest::blocking::get("https://static.etherpad.org/plugins.full.json")
        .unwrap().json::<Plugins>().unwrap();
    println!("{:?}", res);
}


#[derive(Deserialize, Debug)]
pub struct Plugins(HashMap<String,Plugin>);

#[derive(Deserialize, Debug)]
pub struct Plugin {
    name:String,
    description:String,
    time: String,
    version:String,
    official:bool,
    data:  Data
}

#[derive(Deserialize, Debug)]
pub struct Data {
    _id:String,
    _rev:String,
    name:String,
    #[serde(rename = "dist-tags")]
    dist_tags:Option<Value>,
    #[serde(flatten)]
    versions: Option<HashMap<String, Version>>
}

#[derive(Deserialize, Debug, Default)]
pub struct Version {
    id: Option<String>,
    data_id:Option<String>,
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    time: Option<String>,
    author: Option<Author>,
    contributors:Option<Vec<Author>>,
    license: Option<String>,
    keywords:Option<Vec<String>>,
    repository:Option<Repository>,
    bugs: Option<Repository>,
    homepage:Option<String>,
    funding:Option<Vec<Repository>>,
    dependencies:Option<Value>,
    #[serde(rename = "devDependencies")]
    dev_dependencies:Option<Value>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies:Option<Value>,
    #[serde(rename = "eslintConfig")]
    eslint_config:Option<Value>,
    scripts:Option<Value>,
    engines:Option<Value>
}

#[derive(Deserialize, Debug)]
pub struct Author {
    name: String,
    email: String
}
#[derive(Deserialize, Debug)]
pub struct Repository {
    r#type: Option<String>,
    url:String
}