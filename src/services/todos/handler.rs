use crate::domain::todo::{TodoEntity, TodoEntityBuilder};
use crate::modules::db::schema::todos::dsl::*;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use derive_builder::Builder;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::apis::todo::{
    CreateTodoResponse, DeleteTodoResponse, GetTodoResponse, GetTodosResponse, Todo,
    UpdateTodoResponse,
};
use gen_server::models::{
    DeleteTodoPathParams, GetTodoPathParams, GetTodosQueryParams, UpdateTodo, UpdateTodoPathParams,
};
use uuid::Uuid;

#[derive(Clone, Builder)]
pub struct TodoService {
    pool: Pool<AsyncPgConnection>,
}

#[async_trait]
impl Todo for TodoService {
    async fn create_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        UpdateTodo {
            title: body_title,
            description: body_description,
            meta: body_meta,
        }: UpdateTodo,
    ) -> Result<CreateTodoResponse, ()> {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let new_todo = TodoEntityBuilder::default()
            .id(Uuid::new_v4())
            .created_at(None)
            .updated_at(None)
            .title(body_title)
            .description(body_description)
            //.metadata(body_meta.map(serde_json::Value::from))
            .metadata(None)
            .build()
            .expect("Failed to build todo");

        let res: TodoEntity = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result(&mut conn)
            .await
            .expect("Failed to insert todo");

        Ok(CreateTodoResponse::Status201_SuccessfulOperation(
            res.into(),
        ))
    }

    async fn delete_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteTodoPathParams,
    ) -> Result<DeleteTodoResponse, ()> {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let path_id = Uuid::parse_str(&_path_params.id).expect("Failed to parse id");

        diesel::delete(todos.filter(id.eq(path_id)))
            .execute(&mut conn)
            .await
            .expect("Failed to delete todo");

        Ok(DeleteTodoResponse::Status202_SuccessfulOperation)
    }

    async fn get_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        GetTodoPathParams { id: path_id }: GetTodoPathParams,
    ) -> Result<GetTodoResponse, ()> {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let path_id = Uuid::parse_str(&path_id).expect("Failed to parse id");

        let res = todos
            .filter(id.eq(path_id))
            .select(TodoEntity::as_select())
            .first(&mut conn)
            .await
            .expect("Failed to load todo");

        Ok(GetTodoResponse::Status200_SuccessfulOperation(res.into()))
    }

    async fn get_todos(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        GetTodosQueryParams { limit, offset }: GetTodosQueryParams,
    ) -> Result<GetTodosResponse, ()> {
        let mut conn = self.pool.get().await.expect("Failed to get connection");

        let res: Vec<TodoEntity> = todos
            .filter(title.is_not_null())
            .select(TodoEntity::as_select())
            .offset(offset.unwrap_or_else(|| 0) as i64)
            .limit(limit.unwrap_or_else(|| 10) as i64)
            .load(&mut conn)
            .await
            .expect("Failed to load todos");

        let res = res.into_iter().map(|todo| todo.into()).collect();
        Ok(GetTodosResponse::Status200_SuccessfulOperation(res))
    }

    async fn update_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        UpdateTodoPathParams { .. }: UpdateTodoPathParams,
        _body: UpdateTodo,
    ) -> Result<UpdateTodoResponse, ()> {
        todo!("Not implemented");
    }
}

impl AsRef<TodoService> for TodoService {
    fn as_ref(&self) -> &TodoService {
        self
    }
}