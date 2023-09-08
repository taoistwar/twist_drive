use axum::{http::StatusCode, Json};

use std::{fs, path::Path};
use twist_drive_core::{file_hash, CommonResp, FileSign};

use super::{resp_err, resp_ok};

static EMPTY_FILE_SHA2: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub async fn exists_route(Json(payload): Json<FileSign>) -> (StatusCode, Json<CommonResp>) {
    if payload.hash.eq_ignore_ascii_case(EMPTY_FILE_SHA2) {
        return resp_err("source file is empty");
    }
    if payload.hash.len() != EMPTY_FILE_SHA2.len() {
        return resp_err("source file hash error");
    }

    let file = payload.file;
    let path = Path::new(&file);
    // 文件是否存在
    if !path.exists() {
        return resp_err("file not exists");
    }
    if let Ok(meta) = fs::metadata(path) {
        // 类型=file
        if !meta.is_file() {
            return resp_err("server path is dir");
        }
        // 大小
        if meta.len() != payload.size && !meta.is_file() {
            return resp_err("file size not match, need reupload");
        }
    }
    // hash相同
    let hash = file_hash(&file);
    if hash != payload.hash {
        return resp_err("file sign not match, need reupload");
    }

    resp_ok("file had uploaded")
}
