use diesel::sql_types::Integer;
use diesel::sql_types::Text;
use diesel::{AsChangeset, Insertable, OptionalExtension, Queryable, QueryableByName, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use crate::schema::sequences;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

#[derive(Serialize, Deserialize,Insertable, QueryableByName,Queryable, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=sequences)]
pub struct Sequence {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Integer)]
    pub val: i64
}


impl Sequence {
    pub fn new(id: String, val: i64) -> Sequence {
        Sequence {
            id,
            val
        }
    }
    pub fn insert(sequence: Sequence, conn: &mut SqliteConnection) -> Result<Sequence,
        diesel::result::Error> {
        use crate::schema::sequences::dsl::*;
        diesel::insert_into(sequences)
            .values(sequence)
            .get_result(conn)
    }

    pub fn update(sequence: Sequence, conn: &mut SqliteConnection) -> Result<Sequence,
        diesel::result::Error> {
        use crate::schema::sequences::dsl::*;
        diesel::update(sequences)
            .filter(id.eq(sequence.id.clone()))
            .set(sequence)
            .get_result(conn)
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Sequence>, diesel::result::Error> {
        use crate::schema::sequences::dsl::*;
        sequences.load::<Sequence>(conn)
    }

    pub fn get_by_id(id_to_search: String, conn: &mut SqliteConnection) -> Option<Sequence> {
        use crate::schema::sequences::dsl::*;
        sequences.filter(id.eq(id_to_search))
            .first::<Sequence>(conn)
            .optional()
            .unwrap()
    }
}