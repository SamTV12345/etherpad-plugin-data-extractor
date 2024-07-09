use chrono::NaiveDateTime;
use crate::schema::timestamp_sync as tssync;
use diesel::{AsChangeset, ExpressionMethods, Insertable, OptionalExtension, Queryable, QueryableByName, QueryDsl, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use diesel::sql_types::{Text, Timestamp};
use crate::entities::plugin::Plugin;

#[derive(Serialize, Deserialize,Insertable, QueryableByName,Queryable, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=tssync)]
pub struct TimestampSync {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Timestamp)]
    pub timestamp: NaiveDateTime
}


impl TimestampSync {
    pub fn insert(timestamp_sync_insert: TimestampSync, conn: &mut SqliteConnection) -> Result<TimestampSync,
        diesel::result::Error> {
        use crate::schema::timestamp_sync::dsl::*;
        return diesel::insert_into(timestamp_sync)
            .values(timestamp_sync_insert)
            .get_result(conn)
    }

    pub fn get_by_id(id_to_search: &str, conn: &mut SqliteConnection) -> Result<Option<TimestampSync>, diesel::result::Error> {
        use crate::schema::timestamp_sync::dsl::*;
        let opt_timestamp  = timestamp_sync.filter(id.eq(id_to_search)).first::<TimestampSync>(conn).optional();
        return if opt_timestamp.is_err() {
            Err(opt_timestamp.err().unwrap())
        } else {
            Ok(opt_timestamp.unwrap())
        }
    }
}
