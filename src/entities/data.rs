use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use crate::schema::datas;
use diesel::sql_types::{Integer,Text};
#[derive(Serialize, Deserialize, Queryable,Insertable, QueryableByName, Clone)]
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
    #[diesel(sql_type = Text)]
    pub license: String,
    #[diesel(sql_type = Integer)]
    pub downloads: i32
}