use crate::domain::school::SchoolEntity;
use crate::modules::db::schema::schools::dsl::schools;
use app_models::api::api::{Page, PageBuilder, Pagination};
use app_models::models::school::{School, SchoolBuilder};
use app_models::Result;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

pub async fn get_single_school(id: String, pool: Pool<AsyncPgConnection>) -> Result<School> {
    let result = SchoolBuilder::default()
        .id(id)
        .name("School 1".to_string())
        .build()?;

    Ok(result)
}

pub async fn get_schools(
    pagination: Pagination,
    _pool: Pool<AsyncPgConnection>,
) -> Result<Page<School>> {
    // let mut conn = pool.get().await?;
    // let data: Vec<SchoolEntity> = schools::table
    //     // .limit(pagination.per_page)
    //     // .offset(pagination.page * pagination.per_page)
    //     // .select(SchoolEntity::as_select())
    //     .select(schools::all_columns)
    //     .load(&mut conn)
    //     .await?;

    let result = PageBuilder::default()
        .page(pagination.page)
        .per_page(pagination.per_page)
        .total(1)
        .items(vec![
            SchoolBuilder::default()
                .id("1".to_string())
                .name("School 1".to_string())
                .build()
                .unwrap(),
            SchoolBuilder::default()
                .id("1".to_string())
                .name("School 1".to_string())
                .build()
                .unwrap(),
        ])
        .build()?;

    Ok(result)
}
