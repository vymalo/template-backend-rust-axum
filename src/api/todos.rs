use crate::services::todos::handler::TodosService;
use crate::services::utils::pack_cbor;
use axum::async_trait;
use axum::body::Bytes;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::todos::{
    CreateTodoResponse, DeleteTodoResponse, GetTodoResponse, GetTodosResponse, Todos,
    UpdateTodoResponse,
};
use gen_server::apis::todos_test::{GetTodosJsonResponse, GetTodosTestCborResponse, TodosTest};
use gen_server::models::{
    DeleteTodoPathParams, GetTodoPathParams, GetTodosJsonQueryParams, GetTodosQueryParams,
    UpdateTodo, UpdateTodoPathParams,
};

#[async_trait]
impl Todos for TodosService {
    async fn create_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Bytes,
    ) -> Result<CreateTodoResponse, ()> {
        let body = serde_cbor::from_slice(&body.to_vec()).expect("Failed to deserialize body");
        let res = self.create_one(body).await.expect("Failed to create todo");
        let res = pack_cbor(&res).expect("Failed to serialize todo");
        Ok(CreateTodoResponse::Status201_SuccessfulOperation(res))
    }

    async fn delete_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        DeleteTodoPathParams { id }: DeleteTodoPathParams,
    ) -> Result<DeleteTodoResponse, ()> {
        self.delete_one(id).await.expect("Failed to delete todo");
        Ok(DeleteTodoResponse::Status202_SuccessfulOperation)
    }

    async fn get_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        GetTodoPathParams { id }: GetTodoPathParams,
    ) -> Result<GetTodoResponse, ()> {
        let res = self.find_one(id).await.expect("Failed to find todo");
        let res = pack_cbor(&res).expect("Failed to serialize todo");
        Ok(GetTodoResponse::Status200_SuccessfulOperation(res))
    }

    async fn get_todos(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        params: GetTodosQueryParams,
    ) -> Result<GetTodosResponse, ()> {
        let res = self.find_todos(params).await.expect("Failed to find todos");
        let res = pack_cbor(&res).expect("Failed to serialize todos");
        Ok(GetTodosResponse::Status200_SuccessfulOperation(res))
    }

    async fn update_todo(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        UpdateTodoPathParams { id }: UpdateTodoPathParams,
        body: Bytes,
    ) -> Result<UpdateTodoResponse, ()> {
        let body = serde_cbor::from_slice(&body).expect("Failed to deserialize body");
        let res = self
            .update_one(id, body)
            .await
            .expect("Failed to update todo");
        let res = pack_cbor(&res).expect("Failed to serialize todo");
        Ok(UpdateTodoResponse::Status200_SuccessfulOperation(res))
    }
}

#[async_trait]
impl TodosTest for TodosService {
    async fn get_todos_json(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: GetTodosJsonQueryParams,
    ) -> Result<GetTodosJsonResponse, ()> {
        let res = self
            .find_todos(GetTodosQueryParams {
                limit: query_params.limit,
                offset: query_params.offset,
            })
            .await
            .expect("Failed to find todos");
        Ok(GetTodosJsonResponse::Status200_SuccessfulOperation(res))
    }

    async fn get_todos_test_cbor(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetTodosTestCborResponse, ()> {
        let res = UpdateTodo {
            title: "Test".to_string(),
            description: Some("Test".to_string()),
            meta: Some(std::collections::HashMap::new()),
        };
        let res = pack_cbor(&res).expect("Failed to serialize todos");
        Ok(GetTodosTestCborResponse::Status200_SuccessfulOperation(res))
    }
}

impl AsRef<TodosService> for TodosService {
    fn as_ref(&self) -> &TodosService {
        self
    }
}