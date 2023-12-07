use axum::{http::StatusCode, Json};
use log::debug;

use std::{fs, path::Path};
use twist_drive_core::{file_hash_sha_256, CommonResp, FileSign};

use crate::gen_real_path;

use super::{resp_err, resp_ok};

static EMPTY_FILE_SHA2: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub async fn exists_route(Json(payload): Json<FileSign>) -> (StatusCode, Json<CommonResp>) {
    // validate input
    if payload.hash.eq_ignore_ascii_case(EMPTY_FILE_SHA2) {
        debug!("{} not exists", &payload.file);
        return resp_err("source file is empty");
    }
    if payload.hash.len() != EMPTY_FILE_SHA2.len() {
        debug!("{} not exists", &payload.file);
        return resp_err("source file hash error");
    }

    let file = gen_real_path(&payload.file);

    let path = Path::new(&file);
    // 文件是否存在
    if !path.exists() {
        debug!("{} not exists", &payload.file);
        return resp_err("file not exists");
    }
    if let Ok(meta) = fs::metadata(path) {
        // 类型=file
        if !meta.is_file() {
            debug!("{} not exists", &payload.file);
            return resp_err("server path is dir");
        }
        // 大小
        if meta.len() != payload.size && !meta.is_file() {
            debug!("{} not exists", &payload.file);
            return resp_err("file size not match, need reupload");
        }
    }
    // hash相同
    if let Ok(hash) = file_hash_sha_256(&file) {
        if hash != payload.hash {
            debug!("{} not exists", &payload.file);
            return resp_err("file sign not match, need reupload");
        }
    }

    debug!("{} is exists", &payload.file);
    resp_ok("file had uploaded")
}
