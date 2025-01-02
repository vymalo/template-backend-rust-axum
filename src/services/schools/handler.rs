use app_models::api::api::{Page, PageBuilder, Pagination};
use app_models::models::school::{School, SchoolBuilder};

pub async fn get_single_school(id: String) -> School {
    SchoolBuilder::default()
        .id(id)
        .name("School 1".to_string())
        .build()
        .unwrap()
}

pub async fn get_schools(pagination: Pagination) -> Page<School> {
    PageBuilder::default()
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
        .build()
        .unwrap()
}