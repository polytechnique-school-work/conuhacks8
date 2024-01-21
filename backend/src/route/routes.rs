use axum::{routing::get, Router};

use super::day::get_day;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/day/:year/:ordinal", get(get_day))
}
