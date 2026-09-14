use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};

 
#[tokio::main]
async fn main() {
    async fn about() -> &'static str {"About"}
    async fn list_users() -> &'static str {"List Users"}

    async fn send_json() -> Json<serde_json::Value> {
        Json(serde_json::json!({"msg":"hello from server"}))
    }
    
    async fn created(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
        // let id = id.parse::<u64>(); 
        (StatusCode::CREATED, Json(serde_json::json!({"msg": &id.parse::<u64>().unwrap()})))
    }

    let app = Router::new()
    // .route("/", get(|| async { "Hello, World!" }))
    .route("/", get(send_json))
    .route("/about", get(about))
    .route("/users/{id}", get(list_users).post(created)); 
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}