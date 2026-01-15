use axum::{
    extract::Path, extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json,
};
use serde_json::{json, Value};

use crate::{
    app_state::AppState,
    models::city::City,
    requests::request_city::CityRequest,
    resources::city_resource::CityResource,
    services::generic_query::{generic_delete, generic_insert, generic_list, generic_update},
};

pub async fn list_cities(
    State(state): State<AppState>,
    Query(filter): Query<CityResource>,
) -> impl IntoResponse {
    let result = generic_list::<City, CityResource>(filter, Some(("id", "DESC")), &state.db).await;

    match result {
        Ok(data) => (
            StatusCode::OK,
            Json(json!({ "status": "success", "data": data })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
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
        id: 0,
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
pub async fn delete_city(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
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
