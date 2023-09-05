use std::collections::{HashMap, HashSet};
use chrono::{Date, NaiveDate, NaiveDateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use crate::entities::plugin::Plugin as PluginEntity;
use crate::entities::data::Data as DataEntity;
use uuid::Uuid;
pub fn download_current_plugins(){
    let res = reqwest::blocking::get("https://static.etherpad.org/plugins.full.json")
        .unwrap().json::<Plugins>().unwrap();
    res.0.iter().for_each(|(key, value)|{
        let time = value.time.parse::<NaiveDate>().unwrap();
        let n_time:NaiveDateTime = time.and_hms_opt(0,0,0).unwrap();

        let plugin_to_insert = PluginEntity::new(key.to_string(), value.description.clone(), value
                                            .version.clone(), n_time, value.official);
        let data_to_insert = value.data.clone();

        let data_entity =  DataEntity::new(Uuid::new_v4().to_string(), value.name.clone(),
                                                                 data_to_insert._id.clone(),
                                                                 data_to_insert._rev.clone(),
                                           data_to_insert.name.clone(), data_to_insert.license.clone()
                                                                     .unwrap_or("".to_string()),
                                                                 value.downloads);

        if PluginEntity::get_by_name(key.to_string(), &mut crate::db::establish_connection())
            .is_some(){
            let updated_plugin = PluginEntity::update(plugin_to_insert, &mut
            crate::db::establish_connection()).unwrap();

            match DataEntity::get_by_name(updated_plugin.name.clone(), &mut
                crate::db::establish_connection()){
                Some(..) => {
                    let data_updated = DataEntity::update(data_entity, updated_plugin.name
                        .clone(),
                                                          &mut crate::db::establish_connection()).unwrap();


                },
                None => {
                    let data_inserted = DataEntity::insert(data_entity, &mut crate::db::establish_connection()).unwrap();
                }
            }


            return;
        }
        else{
            let plugin_inserted  = PluginEntity::insert(plugin_to_insert, &mut
            crate::db::establish_connection()).unwrap();

        }
    });
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
    data:  Data,
    downloads: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    _id:String,
    _rev:String,
    name:String,
    #[serde(rename = "dist-tags")]
    dist_tags:Option<Value>,
    #[serde(flatten)]
    versions: Option<HashMap<String, Version>>,
    license: Option<String>
}

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug,Clone)]
pub struct Author {
    name: String,
    email: String
}
#[derive(Deserialize, Debug,Clone)]
pub struct Repository {
    r#type: Option<String>,
    url:String
}