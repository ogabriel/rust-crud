use axum::{Router, routing::delete, routing::get, routing::post, routing::put};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let app = Router::new()
        .route("/pet", post(create_pet))
        .route("/pet/:pet_id", get(get_pet))
        .route("/pet", put(update_pet))
        .route("/pet/:pet_id", delete(delete_pet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pet() {}
async fn create_pet() {}
async fn find_pet() {}
async fn find_pet_by_status() {}
async fn update_pet() {}
async fn delete_pet() {}
