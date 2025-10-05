use axum::extract::Path;
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pets", get(list_pets))
        .route("/pets", post(create_pet))
        .route("/pets/{pet_id}", get(find_pet))
        .route("/pets/findByStatus", get(find_pet_by_status))
        .route("/pets/{pet_id}", put(update_pet))
        .route("/pets/{pet_id}", delete(delete_pet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_pets() {}
async fn create_pet() {}
async fn find_pet(Path(pet_id): Path<String>) {}
async fn find_pet_by_status() {}
async fn update_pet(Path(pet_id): Path<String>) {}
async fn delete_pet(Path(pet_id): Path<String>) {}
