use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::Pool;
use slog;

pub struct AppState {
    pub db_pool: Pool,
    pub log: slog::Logger
}

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoList {
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoItem {
    pub title: String
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: bool
}