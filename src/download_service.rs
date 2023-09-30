use std::collections::{HashMap};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use diesel::PgConnection;
use serde::Deserialize;
use serde_json::Value;
use crate::entities::plugin::Plugin as PluginEntity;
use crate::entities::data::Data as DataEntity;
use uuid::Uuid;

pub fn download_current_plugins(){
    let download_url = std::env::var("DOWNLOAD_URL")
        .unwrap_or("https://static.etherpad.org/plugins.full.json".to_string());
    let res = reqwest::blocking::get(download_url)
        .unwrap().json::<Plugins>().unwrap();

    let conn = &mut crate::db::establish_connection();

    res.0.iter().for_each(|(key, value)|{
        let time = value.time.parse::<NaiveDate>().unwrap();
        let n_time:NaiveDateTime = time.and_hms_opt(0,0,0).unwrap();

        let plugin_to_insert = PluginEntity::new(key.to_string(), value.description.clone(), value
                                            .version.clone(), n_time, value.official);
        let data_to_insert = value.data.clone();

        let data_entity =  DataEntity::new(Uuid::new_v4().to_string(), value.name.clone(),
                                                                 data_to_insert._id.clone(),
                                                                 data_to_insert._rev.clone(),
                                           data_to_insert.name.clone(), data_to_insert.license.clone(),
                                                                 value.downloads);

        match PluginEntity::get_by_name(key.to_string(), conn) {
            Some(p)=>{
            let updated_plugin = PluginEntity::update(plugin_to_insert, conn).unwrap();

            let inserted_data = insert_or_update_data_entity(updated_plugin.clone(),
                                                             data_entity.clone(), conn);
            if inserted_data.is_ok() {
                insert_or_update_version_entities(updated_plugin.clone(), inserted_data.unwrap(),
                                                  data_to_insert.versions.clone(), conn, data_to_insert);
            }

            return;
        },
            None=>{
                let plugin_inserted  = PluginEntity::insert(plugin_to_insert, conn).unwrap();
                let inserted_data = insert_or_update_data_entity(plugin_inserted.clone(),
                                                                 data_entity.clone(), conn);
                if inserted_data.is_ok(){
                    insert_or_update_version_entities(plugin_inserted.clone(), inserted_data
                        .unwrap(), data_to_insert.versions.clone(), conn, data_to_insert);
                }
            }
        }
    });
}


fn insert_or_update_data_entity(updated_plugin: crate::entities::plugin::Plugin, data_entity:
DataEntity, conn: &mut PgConnection) -> Result<crate::entities::data::Data, ()>{
    match DataEntity::get_by_name(updated_plugin.name.clone(), &mut
        crate::db::establish_connection()){
        Some(..) => {
            let data_updated = DataEntity::update(data_entity, updated_plugin.name
                .clone(), conn).unwrap();
            Ok(data_updated)
        },
        None => {
            let data_inserted = DataEntity::insert(data_entity, conn).unwrap();
            return Ok(data_inserted)
        }
    }
}


fn insert_or_update_version_entities(updated_plugin: crate::entities::plugin::Plugin,
                                     data_to_insert: crate::entities::data::Data,
                                     versions: Option<HashMap<String, Version>>,
                                     conn: &mut PgConnection,
                                     data_from_json: Data) {
   if let Some(map) = versions{
       map.iter().for_each(|(_,val)|{

           let mut time = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                                             NaiveTime::from_hms_opt(0, 0, 0).unwrap());

           match &data_from_json.time {
                Some(t) => {
                    if t.contains_key(val.version.clone().unwrap().as_str()){
                        let time_1 = t.get(val.version.clone().unwrap().as_str()).unwrap().clone();
                        time = time_1.naive_utc();
                    }
                },
                None => {
                }
           }

            let (name, email) = get_author_info(&val.author);
            let mut opt_rep_type = None;
            let mut opt_rep_url = None;

                if let Some(v) =  val.repository.clone() {
                        opt_rep_type = v.r#type;
                        opt_rep_url = v.url;
                }
           let key = get_version_key(updated_plugin.name.clone(),
                           val
                               .version.clone().unwrap());
           let opt_version_image = get_image_from_readme(data_from_json.readme.clone());
              let version_to_insert = crate::entities::version::Version::new(get_version_key(updated_plugin.name.clone(),
                                                                                             val
                                                                                                 .version.clone().unwrap()),
                                                                             data_to_insert.id.clone(),
                                                                            val.name.clone().unwrap(),
                                                                            val.version.clone().unwrap(),
                                                                            val.description.clone().unwrap(),
                                                                             time,
                                                                             name,
                                                                             email,
                                                                            val.license.clone(),
                                                                            opt_rep_type,
                                                                            opt_rep_url,
              Some(val.keywords.clone().unwrap_or(vec![]).join(",")),
                                                                             opt_version_image,
                                                                             data_from_json.readme
                                                                                 .clone()
              );
              match crate::entities::version::Version::get_by_id(key, conn){
                Some(..) => {
                    let key = get_version_key(updated_plugin.name.clone(),
                                              val
                                                  .version.clone().unwrap());
                    crate::entities::version::Version::update(version_to_insert, conn, key);
                },
                None => {
                    crate::entities::version::Version::insert(version_to_insert, conn).unwrap();
                }
              }
         })
       }
   }


fn get_author_info(option: &Option<AuthorType>) -> (String, String) {
    return match option {
        Some(AuthorType::Author(author)) => {
            return (author.name.clone().unwrap_or("".to_string()), author.email.clone().unwrap_or(""
                .to_string()))
        },
        Some(AuthorType::AuthorString(author)) => {
            let author_split = author.split("<").collect::<Vec<&str>>();
            if author_split.len() > 1 {
                return (author_split[0].to_string(), author_split[1].to_string().replace(">", ""))
            }
            (author_split[0].to_string(), "".to_string())
        },
        None => {
            ("".to_string(), "".to_string())
        }
    }
}


fn get_image_from_readme(readme: Option<String>) -> Option<String> {
    const REGEX: &str = "\\b(https?:\\/\\/[\\S]+?(?:png|jpe?g|gif))\\b";
    let re = regex::Regex::new(REGEX).unwrap();
    let mut image = None;
    if let Some(readme) = readme {
        let caps = re.captures(&readme);
        if let Some(cap) = caps {
            image = Some(cap.get(0).unwrap().as_str().to_string());
        }
    }
    image
}

fn get_version_key(plugin_name: String, version_tag: String) -> String{
    format!("{}_{}", plugin_name, version_tag)
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
    dist_tags: Option<Value>,
    versions: Option<HashMap<String, Version>>,
    license: Option<String>,
    readme: Option<String>,
    time: Option<HashMap<String, DateTime<Utc>>>
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Version {
    id: Option<String>,
    data_id:Option<String>,
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    author: Option<AuthorType>,
    contributors: Option<Contributor>,
    license: Option<String>,
    keywords:Option<Vec<String>>,
    repository:Option<Repository>,
    bugs: Option<Repository>,
    homepage:Option<String>,
    funding:Option<Repository>,
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
#[serde(untagged)]
enum Contributor {
    Author(Vec<Author>),
    Contributors(Vec<Vec<Author>>),
    SingleAuthor(Author)
}
#[derive(Deserialize, Debug,Clone)]
#[serde(untagged)]
enum AuthorType{
    Author(Author),
    AuthorString(String)
}

#[derive(Deserialize, Debug,Clone)]
pub struct Author {
    name: Option<String>,
    email: Option<String>
}
#[derive(Deserialize, Debug,Clone)]
pub struct Repository {
    r#type: Option<String>,
    url:Option<String>
}