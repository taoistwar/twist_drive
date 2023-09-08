mod download_route;
mod exists_route;
mod upload_route;

use axum::{http::StatusCode, Json};
pub use download_route::*;
pub use exists_route::*;

use twist_drive_core::CommonResp;
pub use upload_route::*;

pub fn resp_err(msg: &str) -> (StatusCode, Json<CommonResp>) {
    (
        StatusCode::OK,
        Json(CommonResp {
            status: false,
            msg: msg.into(),
        }),
    )
}

pub fn resp_ok(msg: &str) -> (StatusCode, Json<CommonResp>) {
    (
        StatusCode::OK,
        Json(CommonResp {
            status: true,
            msg: msg.into(),
        }),
    )
}
