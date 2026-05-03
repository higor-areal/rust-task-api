use crate::models::task::Task;

use axum::Json;

pub async fn home() -> String{
"Bem vindo ao servidor do higor".to_string()
}

pub async fn echo(Json(item): Json<Task>) -> Json<Task>{
    Json(item)
}