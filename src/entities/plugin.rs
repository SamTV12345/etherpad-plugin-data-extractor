use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use crate::schema::plugins;
use diesel::sql_types::{Text, Timestamp, Bool};

#[derive(Serialize, Deserialize, Queryable,Insertable, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=plugins)]
pub struct Plugin {
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Text)]
    pub version: String,
    #[diesel(sql_type = Timestamp)]
    pub time: NaiveDateTime,
    #[diesel(sql_type = Bool)]
    pub official: bool
}
