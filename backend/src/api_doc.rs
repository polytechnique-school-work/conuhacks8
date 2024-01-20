use crate::{route::day, schedule};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
        paths(
            day::get_day,
        ),
        components(
            schemas(
                schedule::day_info::DayInfo,
            )
        ),
        tags(
            (name = "TODO", description = "TODO")
        )
    )]
pub struct ApiDoc;
