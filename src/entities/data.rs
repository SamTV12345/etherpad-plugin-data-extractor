use diesel::{Insertable, Queryable, QueryableByName, RunQueryDsl, SqliteConnection, AsChangeset,
             QueryDsl,
             ExpressionMethods, OptionalExtension};
use serde::{Deserialize, Serialize};
use crate::schema::datas;
use diesel::sql_types::{Integer,Text, Nullable};

#[derive(Serialize, Deserialize, Queryable,Insertable, AsChangeset, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Version, foreign_key = plugin_name), )]
#[diesel(table_name=datas)]
pub struct Data {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub plugin_name: String,
    #[diesel(sql_type = Text)]
    pub _id: String,
    #[diesel(sql_type = Text)]
    pub _rev: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub license: Option<String>,
    #[diesel(sql_type = Integer)]
    pub downloads: i32
}

impl Data {
    pub fn new(id: String, plugin_name: String, _id: String, _rev: String, name: String, license:
    Option<String>, downloads: i32) -> Data {
        Data {
            id,
            plugin_name,
            _id,
            _rev,
            name,
            license,
            downloads
        }
    }

    pub fn insert(data_to_insert: Data, conn: &mut SqliteConnection) -> Result<Data,
        diesel::result::Error> {
        use crate::schema::datas::dsl::*;
        diesel::insert_into(datas)
            .values(data_to_insert)
            .get_result(conn)
    }

    pub fn update(data_to_insert : Data,plugin_name_1: String, conn: &mut SqliteConnection) ->
                                                                                      Result<Data, diesel::result::Error> {
        use crate::schema::datas::dsl::*;
        diesel::update(datas)
            .filter(plugin_name.eq(plugin_name_1))
            .set(data_to_insert)
            .get_result(conn)
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Data>, diesel::result::Error> {
        use crate::schema::datas::dsl::*;
        datas.load::<Data>(conn)
    }

    pub fn get_by_name(name_to_search: String, conn: &mut SqliteConnection) -> Option<Data> {
        use crate::schema::datas::dsl::*;
        datas.filter(name.eq(name_to_search))
            .first::<Data>(conn)
            .optional()
            .unwrap()
    }

    pub fn delete_keywords(data_id_to_delete: String, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        diesel::delete(keywords)
            .filter(version_id.eq(data_id_to_delete))
            .execute(conn)
    }
}