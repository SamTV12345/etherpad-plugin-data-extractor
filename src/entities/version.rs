use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use crate::entities::data::Data;
use crate::schema::versions;
use diesel::sql_types::{Text, Timestamp,Bool};

#[derive(Serialize, Deserialize, Queryable,Insertable, QueryableByName, Clone)]
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