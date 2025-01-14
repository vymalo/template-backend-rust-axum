use crate::domain::todo::{TodoEntity, TodoEntityBuilder};
use crate::modules::db::schema::todos::dsl::*;
use anyhow::Result;
use derive_builder::Builder;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::models::{GetTodosQueryParams, Todo, UpdateTodo};

#[derive(Clone, Builder)]
pub struct TodosService {
    pool: Pool<AsyncPgConnection>,
}

impl TodosService {
    pub async fn find_todos(
        &self,
        GetTodosQueryParams { limit, offset }: GetTodosQueryParams,
    ) -> Result<Vec<Todo>> {
        let mut conn = self.pool.get().await?;

        let res = todos
            .filter(title.is_not_null())
            .select(TodoEntity::as_select())
            .offset(offset.unwrap_or_else(|| 0) as i64)
            .limit(limit.unwrap_or_else(|| 10) as i64)
            .load(&mut conn)
            .await?;

        let res = res.into_iter().map(|todo| todo.into()).collect();

        Ok(res)
    }

    pub async fn find_one(&self, todo_id: String) -> Result<Todo> {
        let mut conn = self.pool.get().await?;

        let res = todos
            .filter(id.eq(todo_id))
            .select(TodoEntity::as_select())
            .first(&mut conn)
            .await?;

        Ok(res.into())
    }

    pub async fn create_one(&self, body: UpdateTodo) -> Result<Todo> {
        let mut conn = self.pool.get().await?;
        let new_todo = TodoEntityBuilder::default()
            .id(cuid2::create_id())
            .created_at(None)
            .updated_at(None)
            .title(body.title)
            .description(body.description)
            .metadata(
                body.meta
                    .map(|meta| serde_json::to_value(meta).expect("Failed to serialize meta")),
            )
            .build()?;

        let res: TodoEntity = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result(&mut conn)
            .await?;

        Ok(res.into())
    }

    pub async fn delete_one(&self, todo_id: String) -> Result<()> {
        let mut conn = self.pool.get().await?;

        diesel::delete(todos.filter(id.eq(todo_id)))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn update_one(&self, todo_id: String, body: UpdateTodo) -> Result<Todo> {
        let mut conn = self.pool.get().await?;

        let res: TodoEntity = diesel::update(todos.filter(id.eq(todo_id)))
            .set((
                title.eq(body.title),
                description.eq(body.description),
                metadata.eq(body
                    .meta
                    .map(|meta| serde_json::to_value(meta).expect("Failed to serialize meta"))),
            ))
            .get_result(&mut conn)
            .await?;

        Ok(res.into())
    }
}