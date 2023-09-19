use actix_web::web::Query;
use chrono::{NaiveDateTime};
use diesel::{Insertable, Queryable, QueryableByName, RunQueryDsl, PgConnection, AsChangeset, OptionalExtension, JoinOnDsl, TextExpressionMethods, Table, BoolExpressionMethods, NullableExpressionMethods};
use diesel::dsl::{max};
use serde::{Deserialize, Serialize};
use crate::schema::plugins;
use diesel::sql_types::{Text, Timestamp, Bool};
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::entities::data::Data;
use crate::entities::version::Version;
use crate::package_controller::{AvailableQuery, PluginDto, PluginMetadata, PluginResponse, SortOrder};
use crate::schema::datas::downloads;

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

    pub fn insert(plugin: Plugin, conn: &mut PgConnection) -> Result<Plugin,
        diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        diesel::insert_into(plugins)
            .values(plugin)
            .get_result(conn)
    }

    pub fn update(plugin: Plugin, conn: &mut PgConnection) -> Result<Plugin,
        diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        diesel::update(plugins)
            .filter(name.eq(plugin.name.clone()))
            .set(plugin)
            .get_result(conn)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Plugin>, diesel::result::Error> {
        use crate::schema::plugins::dsl::*;
        plugins.load::<Plugin>(conn)
    }

    pub fn get_by_name(name_to_search: String, conn: &mut PgConnection) -> Option<Plugin> {
        use crate::schema::plugins::dsl::*;
        plugins.filter(name.eq(name_to_search))
            .first::<Plugin>(conn)
            .optional()
            .unwrap()
    }

    pub async fn get_available_plugins(query: Query<AvailableQuery>) -> PluginResponse {
        use crate::schema::plugins::dsl::*;
        use crate::schema::datas::dsl as data_dsl;
        use crate::schema::datas::table as data_table;
        use crate::schema::versions::dsl as versions_dsl;
        use crate::schema::versions as v_alias;

        let (v1,v2)  = diesel::alias!(v_alias as v1, v_alias as v2);

        let subquery
            = v2.select(max(v2.field(versions_dsl::id)))
            .filter(v2.field(versions_dsl::name)
                .eq(v1.field(versions_dsl::name)))
            .group_by(v2.field(versions_dsl::name));

        let conn = &mut crate::db::establish_connection();
        let mut plugins_query = plugins
            .inner_join(data_table.on(name.eq(data_dsl::plugin_name)))
            .left_join(v1.on(name.like(v1.field(versions_dsl::name))))
            .filter(v1.field(versions_dsl::name).is_null()
                        .or(v1.field(versions_dsl::id).nullable().eq_any(subquery)))
            .into_boxed();

        if let Some(q) = query.official.clone() {
            plugins_query = plugins_query.filter(official.eq(q));
        }

        if let Some(q) = &query.query {
            plugins_query = plugins_query.filter(name.like(format!("%{}%", q)));
        }

        if let Some(p) = &query.last_plugin_name {
            plugins_query = plugins_query.filter(name.gt(p));
        }

        match query.page_size {
            Some(p) => plugins_query = plugins_query.limit(p as i64),
            None => plugins_query = plugins_query.limit(50)
        }

        match &query.order {
            None => {
                plugins_query = plugins_query.order(downloads.desc());
            }
            Some(q) => {
                match q {
                    SortOrder::DownloadsASC => {
                        plugins_query = plugins_query.order(data_dsl::downloads.asc());
                    }
                    SortOrder::DownloadsDESC => {
                        plugins_query = plugins_query.order(data_dsl::downloads.desc());
                    }
                    SortOrder::CreatedASC => {
                        plugins_query = plugins_query.order(time.asc());
                    }
                    SortOrder::CreatedDESC => {
                        plugins_query = plugins_query.order(time.desc());
                    }
                    SortOrder::UpdatedASC => {
                        plugins_query = plugins_query.order(v1.field(versions_dsl::time).asc());
                    }
                    SortOrder::UpdatedDESC => {
                        plugins_query = plugins_query.order(v1.field(versions_dsl::time).desc());
                    }
                }
            }
        }

        let res = plugins_query.load::<(Plugin, Data, Option<Version>)>(conn)
            .unwrap();

        let total_plugins = Self::get_total_count().await;
        let total_downloads = Data::get_total_downloads(conn).await;
        let max_downloads = Data::get_lib_with_highest_download(conn).await.unwrap_or(0);
        let metadata = PluginMetadata {
            total_count: total_plugins as i32,
            total_downloads: total_downloads as i32,
            page_size: res.len() as i32,
        };
        let dto = res.iter().map(|(p, d, v)| {
            PluginDto {
                name: p.name.clone(),
                description: p.description.clone(),
                version: v.clone().map_or(p.version.clone(), |v| v.version),
                time: p.time,
                official: p.official,
                popularity_score: d.downloads as f32/ max_downloads as f32,
                author: v.clone().map_or("".to_string(), |v| v.author_name),
                author_email: v.clone().map_or("".to_string(), |v| v.author_email),
                keywords: v.clone().map_or(vec![], |v| v.author_email.split(",").map(|s| s.to_string()).collect::<Vec<String>>()),
                image: v.clone().map_or(None, |v| v.image),
                readme: v.clone().map_or(None, |v| v.readme)
            }}).collect::<Vec<PluginDto>>();

        return PluginResponse {
            metadata,
            plugins: dto
        }
    }


    async fn get_total_count() -> i64 {
        use crate::schema::plugins::dsl::*;

        let res = plugins
            .count()
            .get_result::<i64>(&mut crate::db::establish_connection());
        res.unwrap()
    }
}