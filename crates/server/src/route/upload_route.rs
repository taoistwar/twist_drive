use anyhow::Result;
use axum::{body::Bytes, extract::Multipart, http::StatusCode, Json};
use std::{fs, path::Path};
use twist_drive_core::{bytes_hash, file_hash, CommonResp};

use crate::gen_real_path;
use std::io::prelude::*;

use super::{resp_err, resp_ok};

pub async fn upload_route(files: Multipart) -> (StatusCode, Json<CommonResp>) {
    let (data, name, path, hash, force) = match parse_input(files).await {
        Ok(x) => x,
        Err(msg) => return resp_err(&msg),
    };
    if name.is_none() {
        return resp_err("file field name missing");
    }
    if data.is_none() {
        return resp_err("file field data missing");
    }
    if path.is_none() {
        return resp_err("path field missing");
    }
    if hash.is_none() {
        return resp_err("hash field missing");
    }
    let data = data.unwrap();
    if data.is_empty() {
        return resp_err("file data empty");
    }
    let hash = hash.unwrap();
    if !bytes_hash(&data).eq_ignore_ascii_case(&hash) {
        return resp_err("file data & hash(sha-2) not match");
    }
    let name = name.unwrap();
    let path = path.unwrap();
    let force = force.unwrap();
    match save_file(&name, &path, &data, &hash, force) {
        Ok(_) => resp_ok("had saved"),
        Err(err) => {
            print!("{}", err.to_string());
            resp_err(&(err.to_string() + ", path:" + &path))
        }
    }
}

async fn parse_input(
    mut files: Multipart,
) -> Result<
    (
        Option<Bytes>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<bool>,
    ),
    String,
> {
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
    Ok((data, name, path, hash, force))
}

fn save_file(name: &str, path: &str, data: &Bytes, hash: &str, force: bool) -> anyhow::Result<()> {
    let file = if path.ends_with(name) {
        path.to_owned()
    } else {
        String::from(path) + "/" + name
    };
    let file = gen_real_path(&file);
    let path = Path::new(&file);
    if path.exists() {
        if force {
            if file_hash(&path.to_string_lossy())? != hash {
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
    f.write_all(data)?;
    f.flush()?; // TODO 流式保存

    Ok(())
}
