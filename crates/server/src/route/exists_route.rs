use axum::{http::StatusCode, Json};

use std::{fs, path::Path};
use twist_drive_core::{file_hash, FileSign};

use crate::CommonResp;

use super::{resp_err, resp_ok};

static EMPTY_FILE_MD5: &str = "d41d8cd98f00b204e9800998ecf8427e";

pub async fn exists_route(Json(payload): Json<FileSign>) -> (StatusCode, Json<CommonResp>) {
    if payload.hash.eq_ignore_ascii_case(EMPTY_FILE_MD5) {
        return resp_err("source file is empty");
    }
    if payload.hash.len() != EMPTY_FILE_MD5.len() {
        return resp_err("source file md5 error");
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
    // md5相同
    let md5 = file_hash(&file);
    if md5 != payload.hash {
        return resp_err("file sign not match, need reupload");
    }

    resp_ok("file had uploaded")
}
