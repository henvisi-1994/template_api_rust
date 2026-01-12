use axum::{
    extract::State,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

use crate::{
    app_state::AppState,
    requests::request_city::CityRequest,
    models::city::City,
    services::generic_query::generic_list,
    services::generic_query::generic_insert,
    services::generic_query::generic_update,
    services::generic_query::generic_delete
};

pub async fn list_cities(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let columns = ["id", "name", "region"]; // debe coincidir exactamente con el struct City

    let result = generic_list::<City>(
        "cities",
        &columns,
        Some(("id", "DESC")),
        &state.db,
    )
    .await;

    match result {
        Ok(cities) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": cities
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": e.to_string()
            })),
        ),
    }
}

fn json_response(status: StatusCode, body: Value) -> impl IntoResponse {
    (status, Json(body))
}
#[axum::debug_handler]
pub async fn create_city(
    State(state): State<AppState>,
    Json(body): Json<CityRequest>,
) -> impl IntoResponse {
    let city = City {
    id:0,
    name: body.name,
    region: body.region,
};
    let result = generic_insert(city, &state.db).await;

    match result {
        Ok(city) => json_response(
            StatusCode::CREATED,
            json!({
                "status": "success",
                "data": city
            }),
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "status": "error",
                "message": e.to_string()
            }),
        ),
    }
}
pub async fn update_city(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<CityRequest>,
) -> impl IntoResponse {
    let city = City {
        id,
        name: body.name,
        region: body.region,
    };

    let result = generic_update(city, &state.db).await;

    match result {
        Ok(city) => json_response(
            StatusCode::OK,
            json!({
                "status": "success",
                "data": city
            }),
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "status": "error",
                "message": e.to_string()
            }),
        ),
    }
}
pub async fn delete_city(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = generic_delete::<City, _>(id, &state.db).await;
    match result {
        Ok(city) => json_response(
            StatusCode::OK,
            json!({
                "status": "success",
                "data": city
            }),
        ),
        Err(e) => json_response(
            StatusCode::NOT_FOUND,
            json!({
                "status": "error",
                "message": e.to_string()
            }),
        ),
    }
}
