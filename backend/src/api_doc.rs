use crate::{algorithm, route::day, route::upload, schedule};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
        paths(
            day::get_day,
            day::get_reservation_day,
            upload::upload,
        ),
        components(
            schemas(
                schedule::day_info::DayInfo,
                upload::UploadFile,
                algorithm::day::Day,
            )
        ),
        tags(
            (name = "AutoAdmin", description = "AutoAdmin API"),
        )
    )]
pub struct ApiDoc;
