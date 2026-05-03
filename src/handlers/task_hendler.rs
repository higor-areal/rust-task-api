use crate::models::task::{Task, NewTask};
use crate::responses::response::{ResponseNewTask, ResponseErro};
use crate::state::app_state::AppState;

use std::sync::{
    Arc,
    Mutex
};

use axum::{
    Json,
    extract::State
};

pub async fn home() -> String{
"Bem vindo ao servidor do higor".to_string()
}

pub async fn echo(Json(item): Json<Task>) -> Json<Task>{
    Json(item)
}

pub async fn new_task(State(state): State<Arc<Mutex<AppState>>> , Json(item): Json<NewTask>) -> Result<Json<ResponseNewTask>, Json<ResponseErro>>{
    if !item.valid(){
        return Err(Json(ResponseErro{
            status_code: 400,
            message: "Task inválida".to_string()
        }))
    }
    let mut data = state.lock().unwrap();

    let temp = Task{
        id: data.next_id, 
        title: item.title.clone(), 
        completed: item.completed
    };

    data.next_id +=1;
    data.tasks.push(temp.clone());

    Ok(Json(ResponseNewTask{status_code: 200, task: temp}))
}