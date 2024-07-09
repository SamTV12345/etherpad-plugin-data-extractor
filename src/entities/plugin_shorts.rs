use diesel::{AsChangeset, Insertable, OptionalExtension, Queryable, QueryableByName, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use crate::schema::plugin_shorts;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::sql_types::{Text, Timestamp, Bool, Nullable, Integer};

#[derive(Serialize, Deserialize,Insertable,QueryableByName,Queryable, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=plugin_shorts)]
pub struct PluginShorts {
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub time_downloaded: Option<String>,
    #[diesel(sql_type = Text)]
    pub version: String,
    #[diesel(sql_type = Bool)]
    pub official: bool,
    #[diesel(sql_type = Nullable<Integer>)]
    pub downloads: Option<i32>
}

impl PluginShorts {
    pub fn new(name: String, description: Option<String>, version: String, time_downloaded: Option<String>, official: bool, downloads: Option<i32>) -> PluginShorts {
        PluginShorts {
            name,
            description,
            time_downloaded,
            version,
            official,
            downloads
        }
    }

    pub fn insert(plugin: PluginShorts, conn: &mut SqliteConnection) -> Result<PluginShorts,
        diesel::result::Error> {
        use crate::schema::plugin_shorts::dsl::*;
        diesel::insert_into(plugin_shorts)
            .values(plugin)
            .get_result(conn)
    }

    pub fn update(plugin: PluginShorts, conn: &mut SqliteConnection) -> Result<PluginShorts,
        diesel::result::Error> {
        use crate::schema::plugin_shorts::dsl::*;
        diesel::update(plugin_shorts)
            .filter(name.eq(plugin.name.clone()))
            .set(plugin)
            .get_result(conn)
    }

    pub fn delete(plugin: PluginShorts, conn: &mut SqliteConnection) -> Result<PluginShorts,
        diesel::result::Error> {
        use crate::schema::plugin_shorts::dsl::*;
        diesel::delete(plugin_shorts)
            .filter(name.eq(plugin.name.clone()))
            .get_result(conn)
    }

    pub fn get_by_name(name_to_search: String, conn: &mut SqliteConnection) -> Option<PluginShorts> {
        use crate::schema::plugin_shorts::dsl::*;
        plugin_shorts.filter(name.eq(name_to_search))
            .first::<PluginShorts>(conn)
            .optional()
            .unwrap()
    }
}