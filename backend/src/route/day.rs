use crate::{algorithm::day::Day, schedule::day_info::DayInfo, state::AppState};
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
    Path((_year, ordinal)): Path<(usize, usize)>,
) -> Result<Json<DayInfo>, StatusCode> {
    let schedule = app.schedule.read().await;
    let day_info = schedule
        .get_day(ordinal)
        .ok_or(StatusCode::NOT_FOUND)?
        .get_day_info(schedule.get_day(ordinal - 1).ok_or(StatusCode::NOT_FOUND)?);
    Ok(Json(day_info))
}

#[utoipa::path(
    get,
    path = "/api/reservation/{year}/{ordinal}",
    responses(
        (status = 200, description = "Get daily info", body = Day),
        (status = 404, description = "No info for this day"),
    ),
    params(
        ("year" = usize, Path, description = "Year"),
        ("ordinal" = usize, Path, description = "Day ordinal"),
    ),
)]
pub async fn get_reservation_day(
    State(app): State<AppState>,
    Path((year, ordinal)): Path<(usize, usize)>,
) -> Result<Json<Day>, StatusCode> {
    let schedule = app.schedule.read().await;
    let day = schedule
        .get_day(ordinal)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();
    Ok(Json(day))
}
