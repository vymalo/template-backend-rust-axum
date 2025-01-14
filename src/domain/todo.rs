use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use gen_server::models::Todo;

#[derive(Debug, Queryable, Selectable, Insertable, Builder)]
#[diesel(table_name = crate::modules::db::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoEntity {
    pub id: uuid::Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub metadata: Option<serde_json::Value>,
    pub title: String,
    pub description: Option<String>,
}

impl Into<Todo> for TodoEntity {
    fn into(self) -> Todo {
        Todo {
            id: String::from(self.id),
            created_at: self
                .created_at
                .map(|x| x.and_utc())
                .unwrap_or_else(|| chrono::Utc::now()),
            updated_at: self
                .updated_at
                .map(|x| x.and_utc())
                .unwrap_or_else(|| chrono::Utc::now()),
            meta: self
                .metadata
                .map(|x| serde_json::from_value(x).expect("Failed to deserialize meta")),
            title: self.title,
            description: self.description,
        }
    }
}