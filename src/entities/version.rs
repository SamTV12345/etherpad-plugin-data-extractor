use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, OptionalExtension, Queryable, QueryableByName, RunQueryDsl, PgConnection};
use serde::{Deserialize, Serialize};
use crate::entities::data::Data;
use crate::schema::versions;
use diesel::sql_types::{Text, Timestamp,Bool, Nullable};
use diesel::AsChangeset;
use diesel::QueryDsl;
use diesel::ExpressionMethods;


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
    #[diesel(sql_type = Nullable<Text>)]
    pub license: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub repository_type: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub repository_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub keywords: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub image: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub readme: Option<String>
}

impl Version{
    pub fn new(id: String, data_id: String, name: String, version: String, description: String,
               time: NaiveDateTime, author_name: String, author_email: String, license:
               Option<String>, repository_type: Option<String>, repository_url: Option<String>,
               keywords: Option<String>, image: Option<String>, readme: Option<String>) ->
                                                                                                       Version {
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
            repository_url,
            keywords,
            image,
            readme
        }
    }

    pub fn insert(version_to_insert: Version, conn: &mut PgConnection) -> Result<Version,
        diesel::result::Error> {
        use crate::schema::versions::dsl::*;
        diesel::insert_into(versions)
            .values(version_to_insert)
            .get_result(conn)
    }

    pub fn update(version_to_insert : Version, conn: &mut PgConnection, key: String) ->
                                                                                         Version {
        use crate::schema::versions::dsl::*;
        diesel::update(versions)
            .filter(id.eq(key))
                        .set(version_to_insert)
                        .get_result::<Version>(conn)
            .unwrap()
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Version>, diesel::result::Error> {
        use crate::schema::versions::dsl::*;
        versions.load::<Version>(conn)
    }

    pub fn get_by_id(id_to_search: String, conn: &mut PgConnection) -> Option<Version> {
        use crate::schema::versions::dsl::*;
        use crate::schema::versions::dsl::id as v_id;
        versions.filter(v_id.eq(id_to_search))
            .first::<Version>(conn)
            .optional()
            .unwrap()
    }

    pub fn get_by_name(name_to_search: String, conn: &mut PgConnection) -> Option<Version> {
        use crate::schema::versions::dsl::*;
        use crate::schema::versions::dsl::name as v_name;
        versions.filter(v_name.eq(name_to_search))
            .first::<Version>(conn)
            .optional()
            .unwrap()
    }
}