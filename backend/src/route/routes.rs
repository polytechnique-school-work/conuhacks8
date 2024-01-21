use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::day::{get_day, get_reservation_day};
use super::upload::upload;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/day/:year/:ordinal", get(get_day))
        .route("/upload", post(upload))
        .route("/reservation/:year/:ordinal", get(get_reservation_day))
}
