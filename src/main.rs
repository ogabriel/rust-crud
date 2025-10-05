use sqlx::postgres::PgPoolOptions;

use axum::extract::Path;
use axum::extract::State;
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};

#[derive(Clone)]
struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/rust-crud")
        .await?;

    let state = AppState { pool: pool };

    let app = Router::new()
        .route("/pets", get(list_pets))
        .route("/pets", post(create_pet))
        .route("/pets/{pet_id}", get(find_pet))
        .route("/pets/findByStatus", get(find_pet_by_status))
        .route("/pets/{pet_id}", put(update_pet))
        .route("/pets/{pet_id}", delete(delete_pet))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn list_pets() {}
async fn create_pet() {}
async fn find_pet(State(state): State<AppState>, Path(pet_id): Path<String>) {
    sqlx::query!("SELECT 1", pet_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();
}
async fn find_pet_by_status() {}
async fn update_pet(Path(pet_id): Path<String>) {}
async fn delete_pet(Path(pet_id): Path<String>) {}
