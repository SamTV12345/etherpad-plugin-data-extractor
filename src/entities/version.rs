use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use crate::entities::data::Data;
use crate::schema::versions;
use diesel::sql_types::{Text, Timestamp,Bool};
use diesel::AsChangeset;
#[derive(Serialize, Deserialize, Queryable,Insertable, AsChangeset, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Data))]
#[diesel(table_name=versions)]
pub struct Version {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub data_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub version: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Timestamp)]
    pub time: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub author_name: String,
    #[diesel(sql_type = Text)]
    pub author_email: String,
    #[diesel(sql_type = Text)]
    pub license: String,
    #[diesel(sql_type = Text)]
    pub repository_type: String,
    #[diesel(sql_type = Text)]
    pub repository_url: String
}

impl Version{
    pub fn new(id: String, data_id: String, name: String, version: String, description: String, time: NaiveDateTime, author_name: String, author_email: String, license: String, repository_type: String, repository_url: String) -> Version {
        Version {
            id,
            data_id,
            name,
            version,
            description,
            time,
            author_name,
            author_email,
            license,
            repository_type,
            repository_url
        }
    }

    pub fn insert(version_to_insert: Version, conn: &mut SqliteConnection) -> Result<Version,
        diesel::result::Error> {
        use crate::schema::versions::dsl::*;
        diesel::insert_into(versions)
            .values(version_to_insert)
            .get_result(conn)
    }

    pub fn update(version_to_insert : Version, conn: &mut SqliteConnection) -> Result<Version,
        diesel::result::Error> {
        use crate::schema::versions::dsl::*;
        diesel::update(versions)
            .set(version_to_insert)
            .get_result(conn)
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Version>, diesel::result::Error> {
        use crate::schema::versions::dsl::*;
        versions.load::<Version>(conn)
    }
}