mod handlers;
mod models;
mod responses;
mod state;

use handlers::task_hendler::{home, echo, new_task, get_tasks};
use axum::{
    Router,
    routing::{get, post}
};
use std::sync::{
    Arc,
    Mutex
};

use crate::state::app_state::AppState;


#[tokio::main]
async fn main() {
    let state = AppState{tasks: vec![], next_id: 0};
    let shared = Arc::new(Mutex::new(state));

    let app = Router::new()
    .route("/", get(home))
    .route("/echo", post(echo))
    .route("/tasks", post(new_task))
    .route("/tasks", get(get_tasks))
    .with_state(shared);

    println!("Start server");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    

    axum::serve(listener, app).await.unwrap();
}   

