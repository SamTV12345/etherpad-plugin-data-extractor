use std::collections::{HashMap, HashSet};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use crate::entities::plugin::Plugin as PluginEntity;
use crate::entities::data::Data as DataEntity;
use uuid::Uuid;
use diesel::SqliteConnection;
use crate::constants::REPO_SYNC;
use crate::db::establish_connection;
use crate::entities::official_repository::OfficialRepository;
use crate::entities::timestamp_sync::TimestampSync;
use changes_stream2::{ChangesStream, Event};
use futures_util::StreamExt;
use crate::api::download_stat::DownloadStat;
use crate::api::replicate_response::ReplicateResponse;
use crate::{db, download_service};
use crate::entities::plugin_shorts::PluginShorts;
use crate::entities::sequence::Sequence;

pub fn load_changes_with_docs(mut seq: i64, conn: &mut SqliteConnection) {
    seq = seq -1;
    println!("Loading changes with docs since: {}", seq);
    let data = reqwest::blocking::get(format!("https://replicate.npmjs\
    .com/_changes?descending=false&since={}&include_docs=true", seq)).unwrap()
        .json::<ReplicateResponse>().unwrap();
    if let Some(data) = data.results.first() {
        let latest_version = data.doc.versions.get(&data.doc.dist_tags.latest.clone());
        match latest_version {
            Some(v)=>{
                if let Some(deprecated) = v.deprecated {
                    if deprecated {
                        println!("{} is deprecated", data.doc.name);
                        PluginShorts::get_by_name(data.doc.name.clone(), conn).map(|plugin| {
                            let _ = PluginShorts::delete(plugin, conn);
                        });
                    }
                } else {
                    println!("{} is not deprecated", data.doc.name);
                    let found_plugin = PluginShorts::get_by_name(data.doc.name.clone(), conn);
                    let is_plugin_official = OfficialRepository::get_by_id(data.doc.name.clone(), conn);
                    let official = is_plugin_official.is_some();
                    let plugin = PluginShorts::new(data.doc.name.clone(), data.doc.description.clone(),
                                                   data.doc.time.modified.clone(), Option::from(data.doc.dist_tags.latest.clone()),
                                                   official, None);
                    match found_plugin {
                        Some(p)=>{
                            let _ = PluginShorts::update(plugin, conn);
                        },
                        None=>{
                            let _ = PluginShorts::insert(plugin, conn);
                        }
                    }
                }
            },
            None=>{
                println!("No latest version found");
            }
        }
    }
}


fn load_download_stats(plugin_names:  Vec<String>) {
    let plugin_lists = plugin_names.join(",");
    let res = reqwest::blocking::get(format!("https://api.npmjs.org/downloads/point/last-month/{}", plugin_lists))
        .unwrap().json::<HashMap<String, DownloadStat>>().unwrap();

}


async fn get_ether_repository_list(page: i32) -> Vec<OfficialRepository> {
    let result = octocrab::instance()
        .orgs("ether")
        .list_repos()
        .per_page(100)
        .page(page as u32)
        .send()
        .await
        .unwrap();
    let mut vec_of_repos = Vec::new();
    for rp in result {
        vec_of_repos.push(OfficialRepository{
            id: rp.name,
        })
    }
    return vec_of_repos;
}

fn days_between(dt1: NaiveDateTime, dt2: NaiveDateTime) -> i64 {
    let duration = if dt1 > dt2 {
        dt1.signed_duration_since(dt2)
    } else {
        dt2.signed_duration_since(dt1)
    };
    duration.num_days()
}


pub async fn get_from_change_api(conn: &mut  SqliteConnection) {
    let mut url = "https://replicate.npmjs.com/_changes?include_docs=false".to_string();
    if let Some(found_seq) = Sequence::get_by_id("sequence".to_string(), conn) {
        url.push_str(&format!("&since={}", found_seq.val));
    }

    let mut changes = ChangesStream::new(url).await.unwrap();
    let mut ticker: i64 = 0;
    while let Some(event) = changes.next().await {
        match event {
            Ok(Event::Change(change)) => {
                ticker += 1;
                if change.id.starts_with("ep_") && change.deleted == true {
                    println!("Deleted: {}", change.id);
                    PluginShorts::get_by_name(change.id.clone(), conn).map(|plugin| {
                        let _ = PluginShorts::delete(plugin, conn);
                    });
                } else if change.id.starts_with("ep_") {
                    load_changes_with_docs(change.seq.as_i64().unwrap(),
                    conn)
                }

                let sequence_num = change.seq.as_i64().unwrap();
                if ticker % 1000 == 0 {
                    let seq_id = Sequence::new("sequence".to_string(), sequence_num);
                    if let Some(_) = Sequence::get_by_id("sequence".to_string(),
                    conn) {
                        let _ = Sequence::update(seq_id, conn);
                    } else {
                        Sequence::insert(seq_id, conn).unwrap();
                    }
                }
            },
            Ok(Event::Finished(finished)) => println!("Finished: {}", finished.last_seq),
            Err(err) => println!("Error: {:?}", err),
        }
    }
}

pub(crate) async fn start_sync() {
    let mut sqlite_conn = establish_connection();
    start_sync_from_github(&mut sqlite_conn).await;
    get_from_change_api(&mut sqlite_conn).await;
}


pub async fn start_sync_from_github(conn: &mut SqliteConnection) {
    let found_timestamp = TimestampSync::get_by_id(REPO_SYNC, conn).unwrap();
    let now_utc: DateTime<Utc> = Utc::now();
    let naive_now: NaiveDateTime = now_utc.naive_utc();

    if let Some(ft) = found_timestamp {
        if days_between(naive_now, ft.timestamp) > 1 {
            handle_repository_sync(conn).await;
            let _ = TimestampSync::insert(TimestampSync{
                id: REPO_SYNC.to_string(),
                timestamp: naive_now,
            }, conn);
        }
    } else {
        handle_repository_sync(conn).await;
        let _ = TimestampSync::insert(TimestampSync{
            id: REPO_SYNC.to_string(),
            timestamp: naive_now,
        }, conn);
    }
}


async fn handle_repository_sync(sqlite_conn: &mut SqliteConnection) {
    let mut first_page = get_ether_repository_list(1).await;
    let mut second_page = get_ether_repository_list(2).await;

    first_page.append(&mut second_page);
    let saved_repos = OfficialRepository::list_all_repos(sqlite_conn).unwrap();
    let saved_repos_hashed: HashSet<String> = saved_repos.iter().map(|e|e.id.clone()).collect();

    let online_repos: HashSet<String> = first_page.iter().map(|e|e.id.clone()).collect();

    first_page.iter().for_each(|p| {
        let is_contained = saved_repos_hashed.contains(&p.id);
        if !is_contained {
            OfficialRepository::insert(p.id.clone(), sqlite_conn).unwrap();
        }
    });

    saved_repos.iter().for_each(|p| {
        let is_contained = online_repos.contains(&p.id);
        if !is_contained {
            OfficialRepository::delete(p.id.clone(), sqlite_conn).unwrap();
        }
    });
}


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
DataEntity, conn: &mut SqliteConnection) -> Result<crate::entities::data::Data, ()>{
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
                                     conn: &mut SqliteConnection,
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