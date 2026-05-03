use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use axum::{
    Json, Router, extract::{State, Path}, routing::{get, post, patch}
};

#[derive(Deserialize, Serialize, Clone)]
struct Task{
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct ResponseErro{
    status: u32,
    mesage: String,
}
#[derive(Serialize)]
struct ResponseNewTask{
    status: u32,
    task: Task,
}

impl Task{
    fn valid(self: &Self) -> bool{
        if self.completed || self.title.is_empty() { return false;}
        true
    }
}
struct AppState{
    tasks: Vec<Task>,
    next_id: u32,
}


#[tokio::main]
async fn main() {
    let state = AppState{tasks: vec![], next_id: 0};
    let shared = Arc::new(Mutex::new(state));

    let app = Router::new()
    .route("/", get(home))
    .route("/task", post(new_task))
    .route("/tasks", get(get_tasks))
    .route("/tasks/{id}/complete", patch(complete_task))
    .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Start server 3000");

    axum::serve(listener, app).await.unwrap();

   
}

async fn home() -> String{
    "Seja bem vindo a API do higor".to_string()
}

async fn new_task(State(state): State<Arc<Mutex<AppState>>>, Json(mut x): Json<Task>) -> Result<Json<ResponseNewTask>, Json<ResponseErro>> {
    
    if !x.valid(){  
        let res_err = ResponseErro{status: 403, mesage: String::from("Invalide")};
        return Err(Json(res_err));
    }

    let mut data = state.lock().unwrap();
    x.id = data.next_id;
    data.next_id+=1;

    data.tasks.push(x.clone());
    let res = ResponseNewTask{status: 200, task: x};

    Ok(Json(res))
}
async fn get_tasks(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<Task>>{
    let data = state.lock().unwrap();
    Json(data.tasks.clone())
}

async fn complete_task(State(state): State<Arc<Mutex<AppState>>>, Path(id): Path<u32>) -> Result<Json<Task>, Json<ResponseErro>>{
    let mut data = state.lock().unwrap();
    if let Some(x) = data.tasks.iter_mut().find(|p| p.id == id){
        if x.completed {
            let msg = ResponseErro{status: 400, mesage: "Task já completa".to_string()};
            return Err(Json(msg));
        }
        x.completed = true;
        return Ok(Json(x.clone()));
    }
    let msg = ResponseErro{status: 400, mesage: "Não encontrada".to_string()};
    Err(Json(msg))
}
