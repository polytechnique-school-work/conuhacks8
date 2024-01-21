use axum::{body::Bytes, extract::State};
use hyper::StatusCode;
use serde::Deserialize;
use std::fs;
use std::io::Write; // bring trait into scope
use std::str;
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadFile {
    content: String,
}

#[utoipa::path(
    get,
    path = "/api/upload",
    responses(
        (status = 200, description = "Upload new datafile"),
        (status = 500, description = "Internal server error"),
    ),
    request_body = UploadFile,
)]
pub async fn upload(State(mut app): State<AppState>, config: Bytes) -> StatusCode {
    if let Ok(data) = str::from_utf8(config.as_ref()) {
        let data: Result<UploadFile, _> = serde_json::from_str(data);
        if let Ok(data) = data {
            let config = data.content.as_bytes();
            let mut file = fs::File::create("new_datafile.csv").unwrap();
            if file.write_all(config).is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        } else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
    app.update_from_file("new_datafile.csv");
    StatusCode::OK
}
