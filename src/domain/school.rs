use diesel::{Queryable, Selectable};
use derive_builder::Builder;

#[derive(Debug, Queryable, Selectable, Builder)]
#[diesel(table_name = crate::modules::db::schema::schools)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SchoolEntity {
    pub id: String,
    pub name: String,
}