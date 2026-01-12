use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::controllers::{
    city_controller::create_city,
    city_controller::update_city,
    city_controller::delete_city,
    city_controller::list_cities,
};
use crate::AppState;
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/cities", get(list_cities))
        .route("/city", post(create_city))
        .route("/city/:id", put(update_city))
        .route("/city/:id", delete(delete_city))
        .with_state(state)
}

