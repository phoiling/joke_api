use axum::{Json, Router, response::IntoResponse, routing::{future::RouteFuture, get}, serve::Listener};
use serde_json::json;

#[tokio::main]
async fn main(){

    let app = Router::new().route("/api", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("sever started");
    axum::serve(listener, app).await.unwrap();
 
}

async fn hello_world() -> impl IntoResponse {
    let json_responce = json!({
        "poo_size": "medum",
        "message": "your a poo",
    });
    Json(json_responce)
}