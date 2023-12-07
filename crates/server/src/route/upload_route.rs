use anyhow::Result;
use axum::{body::Bytes, extract::Multipart, http::StatusCode, Json};
use std::{fmt::Display, fs, path::Path};
use twist_drive_core::{bytes_hash_sha_256, file_hash_sha_256, CommonResp};

use crate::{gen_real_path, EMPTY_FILE_SHA2};
use std::io::prelude::*;

use super::{resp_err, resp_ok};

struct UploadBody {
    name: Option<String>,
    path: Option<String>,
    hash: Option<String>,
    force: Option<bool>,
}
impl UploadBody {
    fn new(
        name: Option<String>,
        path: Option<String>,
        hash: Option<String>,
        force: Option<bool>,
    ) -> UploadBody {
        UploadBody {
            name,
            path,
            hash,
            force,
        }
    }
}
impl Display for UploadBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name:{}, path:{}, hash:{}, force:{}",
            self.name.as_ref().unwrap_or(&"".to_string()),
            self.path.as_ref().unwrap_or(&"".to_string()),
            self.hash.as_ref().unwrap_or(&"".to_string()),
            self.force.unwrap_or(false)
        )
    }
}

pub async fn upload_route(files: Multipart) -> (StatusCode, Json<CommonResp>) {
    let (data, body) = match parse_input(files).await {
        Ok(x) => x,
        Err(msg) => return resp_err(&msg),
    };
    if body.name.is_none() {
        let msg = format!("file field name missing, {}", body);
        return resp_err(&msg);
    }

    if body.path.is_none() {
        let msg = format!("file field path missing, {}", body);
        return resp_err(&msg);
    }
    let path: String = body.path.clone().unwrap();
    if body.hash.is_none() {
        let msg = format!("file field hash missing, {}", body);
        return resp_err(&msg);
    }
    let hash = body.hash.clone().unwrap();
    let name = body.name.clone().unwrap();
    let force = body.force.clone().unwrap();
    if hash.eq_ignore_ascii_case(EMPTY_FILE_SHA2) {
        // 忽略 data 验证
        if let Some(x) = data {
            if x.len() != 0 {
                let msg = format!("data of empty file must empty, {}", body);
                return resp_err(&msg);
            }
        }
        match save_file(&name, &path, None, &hash, force, true) {
            Ok(_) => resp_ok("had saved"),
            Err(err) => {
                print!("{}", err.to_string());
                resp_err(&(err.to_string() + ", path:" + &path))
            }
        }
    } else {
        if data.is_none() {
            let msg = format!("file field data missing, {}", body);
            return resp_err(&msg);
        }
        let data = data.unwrap();
        if !bytes_hash_sha_256(&data).eq_ignore_ascii_case(&hash) {
            let msg = format!("file data & hash not match, {}", body);
            return resp_err(&msg);
        }
        match save_file(&name, &path, Some(data), &hash, force, false) {
            Ok(_) => resp_ok("had saved"),
            Err(err) => {
                print!("{}", err.to_string());
                resp_err(&(err.to_string() + ", path:" + &path))
            }
        }
    }
}

async fn parse_input(mut files: Multipart) -> Result<(Option<Bytes>, UploadBody), String> {
    let mut data: Option<Bytes> = None;
    let mut name: Option<String> = None;
    let mut path: Option<String> = None;
    let mut hash: Option<String> = None; // sha-2
    let mut force: Option<bool> = None;

    while let Some(file) = files.next_field().await.unwrap() {
        let category = file.name().unwrap().to_string();
        if category.eq_ignore_ascii_case("file") {
            name = Some(file.file_name().unwrap().to_string());
            data = Some(file.bytes().await.unwrap());
        } else if category.eq_ignore_ascii_case("path") {
            match file.text().await {
                Ok(text) => {
                    path = Some(text);
                }
                Err(_e) => return Err("path parse fail".into()),
            }
        } else if category.eq_ignore_ascii_case("hash") {
            match file.text().await {
                Ok(text) => {
                    hash = Some(text);
                }
                Err(_e) => return Err("path parse fail".into()),
            }
        } else if category.eq_ignore_ascii_case("force") {
            match file.text().await {
                Ok(text) => {
                    force = Some(text.eq_ignore_ascii_case("true"));
                }
                Err(_e) => return Err("path parse fail".into()),
            }
        } else {
            return Err(format!("unknown field:{category}"));
        }
    }
    let body = UploadBody::new(name, path, hash, force);
    Ok((data, body))
}

fn save_file(
    name: &str,
    path: &str,
    data: Option<Bytes>,
    hash: &str,
    force: bool,
    is_empty_file: bool,
) -> anyhow::Result<()> {
    let file = if path.ends_with(name) {
        path.to_owned()
    } else {
        String::from(path) + "/" + name
    };
    let file = gen_real_path(&file);
    let path = Path::new(&file);
    if path.exists() {
        if force {
            if file_hash_sha_256(&path.to_string_lossy())? != hash {
                // 如果hash相同, 就不需要再保存了
                fs::remove_file(path)?;
            }
        } else {
            return Err(anyhow::anyhow!("file exists"));
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut f = fs::File::create(&path)?;
    if !is_empty_file {
        f.write_all(&data.unwrap())?;
        f.flush()?; // TODO 流式保存
    }
    Ok(())
}
