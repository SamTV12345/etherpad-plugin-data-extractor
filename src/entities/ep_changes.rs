use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use crate::schema::ep_changes as epChanges;
use diesel::sql_types::{Text, Integer};

#[derive(Serialize, Deserialize, Queryable,Insertable, AsChangeset, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=epChanges)]
pub struct EPChange {
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Integer)]
    pub seq_id: i64
}

impl EPChange {
    pub fn insert(change: EPChange, conn: &mut SqliteConnection) -> Result<EPChange, diesel::result::Error> {
        use crate::schema::ep_changes::dsl::*;
        return diesel::insert_into(ep_changes).values(change)
            .get_result::<EPChange>(conn);
    }
}