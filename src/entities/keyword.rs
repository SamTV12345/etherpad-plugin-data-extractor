use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use crate::entities::version::Version;
use crate::schema::keywords;
use diesel::sql_types::{Integer,Text};

#[derive(Serialize, Deserialize, Queryable,Insertable, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Version))]
#[diesel(table_name=keywords)]
pub struct Keyword {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub version_id: String,
    #[diesel(sql_type = Text)]
    pub keyword: String
}