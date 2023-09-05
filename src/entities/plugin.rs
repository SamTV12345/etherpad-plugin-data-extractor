use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName, RunQueryDsl, SqliteConnection, AsChangeset, OptionalExtension};
use serde::{Deserialize, Serialize};
use crate::schema::plugins;
use diesel::sql_types::{Text, Timestamp, Bool};
use diesel::QueryDsl;
use diesel::ExpressionMethods;

#[derive(Serialize, Deserialize,Insertable, QueryableByName,Queryable, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=plugins)]
pub struct Plugin {
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub description: String,

    #[diesel(sql_type = Timestamp)]
    pub time: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub version: String,
    #[diesel(sql_type = Bool)]
    pub official: bool
}


impl Plugin {
    pub fn new(name: String, description: String, version: String, time: NaiveDateTime, official: bool) -> Plugin {
        Plugin {
            name,
            description,
            version,
            time,
            official
        }
    }

    pub fn insert(plugin : Plugin, conn: &mut SqliteConnection) -> Result<Plugin,
        diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        diesel::insert_into(plugins)
            .values(plugin)
            .get_result(conn)
    }

    pub fn update(plugin : Plugin, conn: &mut SqliteConnection) -> Result<Plugin,
        diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        diesel::update(plugins)
            .filter(name.eq(plugin.name.clone()))
            .set(plugin)
            .get_result(conn)
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Plugin>, diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        plugins.load::<Plugin>(conn)
    }

    pub fn get_by_name(name_to_search: String, conn: &mut SqliteConnection) -> Option<Plugin> {
        use crate::schema::plugins::dsl::*;
        plugins.filter(name.eq(name_to_search))
            .first::<Plugin>(conn)
            .optional()
            .unwrap()
    }
}
