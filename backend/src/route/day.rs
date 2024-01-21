use crate::{schedule::day_info::DayInfo, state::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;

#[utoipa::path(
    get,
    path = "/api/day/{year}/{ordinal}",
    responses(
        (status = 200, description = "Get daily info", body = DayInfo),
        (status = 404, description = "No info for this day"),
    ),
    params(
        ("year" = usize, Path, description = "Year"),
        ("ordinal" = usize, Path, description = "Day ordinal"),
    ),
)]
pub async fn get_day(
    State(app): State<AppState>,
    Path((year, ordinal)): Path<(usize, usize)>,
) -> Result<Json<DayInfo>, StatusCode> {
    let day_info = app
        .schedule
        .read()
        .await
        .get_day(ordinal)
        .ok_or(StatusCode::NOT_FOUND)?
        .get_day_info();
    Ok(Json(day_info))
}
