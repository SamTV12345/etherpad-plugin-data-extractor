use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, RunQueryDsl, SqliteConnection,
             QueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};
use crate::entities::version::Version;
use crate::schema::keywords;
use diesel::sql_types::{Text};

#[derive(Serialize, Deserialize, Queryable,Insertable, AsChangeset, QueryableByName, Clone)]
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

impl Keyword{
    pub fn new(id: String, version_id: String, keyword: String) -> Keyword {
        Keyword {
            id,
            version_id,
            keyword
        }
    }

    pub fn insert(keyword_to_insert: Keyword, conn: &mut SqliteConnection) -> Result<Keyword,
        diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        diesel::insert_into(keywords)
            .values(keyword_to_insert)
            .get_result(conn)
    }

    pub fn update(keyword_to_insert : Keyword, conn: &mut SqliteConnection) -> Result<Keyword,
        diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        diesel::update(keywords)
            .set(keyword_to_insert)
            .get_result(conn)
    }

    pub fn delete(keyword_to_delete : Keyword, conn: &mut SqliteConnection) -> Result<usize,
        diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        diesel::delete(keywords)
            .filter(keyword.eq(keyword_to_delete.keyword.clone()))
            .execute(conn)
    }

    pub fn get_by_version_id(version_id_to_search: String, conn: &mut SqliteConnection) -> Result<Vec<Keyword>, diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        keywords.filter(version_id.eq(version_id_to_search))
            .load::<Keyword>(conn)
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Keyword>, diesel::result::Error> {
        use crate::schema::keywords::dsl::*;
        keywords.load::<Keyword>(conn)
    }
}