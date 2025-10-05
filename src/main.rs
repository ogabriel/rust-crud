use sqlx::FromRow;
use sqlx::postgres::PgPoolOptions;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Json;
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};

use serde::{Deserialize, Serialize};

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

#[derive(FromRow, Serialize, Deserialize)]
struct Pet {
    id: i32,
    name: String,
    status: String,
}

async fn list_pets(State(state): State<AppState>) -> Json<Vec<Pet>> {
    Json(
        sqlx::query_as!(Pet, "SELECT id, name, status FROM pets")
            .fetch_all(&state.pool)
            .await
            .unwrap(),
    )
}
#[derive(Deserialize)]
struct PetRequest {
    name: String,
    status: String,
}

async fn create_pet(State(state): State<AppState>, Json(pet): Json<PetRequest>) {
    sqlx::query!(
        "INSERT INTO pets (name, status) VALUES ($1, $2)",
        pet.name,
        pet.status
    )
    .execute(&state.pool)
    .await
    .unwrap();
}
async fn find_pet(State(state): State<AppState>, Path(pet_id): Path<i32>) -> Json<Pet> {
    Json(
        sqlx::query_as!(
            Pet,
            "SELECT id, name, status FROM pets WHERE id = $1",
            pet_id
        )
        .fetch_one(&state.pool)
        .await
        .unwrap(),
    )
}

#[derive(Deserialize)]
struct PetParams {
    status: String,
}

async fn find_pet_by_status(
    State(state): State<AppState>,
    Query(params): Query<PetParams>,
) -> Json<Vec<Pet>> {
    Json(
        sqlx::query_as!(
            Pet,
            "SELECT id, name, status FROM pets WHERE status = $1",
            params.status
        )
        .fetch_all(&state.pool)
        .await
        .unwrap(),
    )
}
async fn update_pet(
    Path(pet_id): Path<i32>,
    State(state): State<AppState>,
    Json(pet): Json<PetRequest>,
) {
    sqlx::query!(
        "UPDATE pets SET name = $1, status = $2 WHERE id = $3",
        pet.name,
        pet.status,
        pet_id
    )
    .execute(&state.pool)
    .await
    .unwrap();
}

async fn delete_pet(Path(pet_id): Path<i32>, State(state): State<AppState>) {
    sqlx::query!("DELETE FROM pets WHERE id = $1", pet_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

