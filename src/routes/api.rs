use axum::{
    middleware,
    Router,
    routing::{delete, get, post, put},
};

use crate::controllers::{
    auth_controller,
    city_controller::create_city,
    city_controller::update_city,
    city_controller::delete_city,
    city_controller::list_cities,
};
use crate::middleware::auth::auth_middleware;
use crate::AppState;
pub fn routes(state: AppState) -> Router {
    let protected = Router::new()
        .route("/auth/me", get(auth_controller::me))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/cities", get(list_cities))
        .route("/city", post(create_city))
        .route("/city/:id", put(update_city))
        .route("/city/:id", delete(delete_city))
        .route("/auth/register", post(auth_controller::register))
        .route("/auth/login", post(auth_controller::login))
        .merge(protected)
        .with_state(state)
}

