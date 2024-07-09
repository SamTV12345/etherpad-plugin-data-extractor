use diesel::{AsChangeset, ExpressionMethods, Insertable, Queryable, QueryableByName, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::SqliteConnection;
use crate::schema::officialRepositories as ofRepo;
use diesel::sql_types::Text;

#[derive(Serialize, Deserialize, Queryable,Insertable, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=ofRepo)]
pub struct OfficialRepository {
    #[diesel(sql_type = Text)]
    pub(crate) id: String
}


impl OfficialRepository {
    pub fn insert(plugin_name: String, conn: &mut SqliteConnection) -> Result<OfficialRepository, diesel::result::Error> {
        use crate::schema::officialRepositories::dsl::*;
        diesel::insert_into(officialRepositories)
            .values(OfficialRepository{
                id: plugin_name
            })
            .get_result(conn)
    }

    pub fn list_all_repos(conn: &mut SqliteConnection) -> Result<Vec<OfficialRepository>, diesel::result::Error> {
        use crate::schema::officialRepositories::dsl::*;
        return officialRepositories.load(conn)
    }

    pub fn delete(plugin_name: String, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::officialRepositories::dsl::*;
       let result = diesel::delete(officialRepositories).filter(id.eq(plugin_name)).execute(conn);
        if result.is_ok() {
            Ok(())

        } else {
            return Err(result.err().unwrap())
        }
    }
}