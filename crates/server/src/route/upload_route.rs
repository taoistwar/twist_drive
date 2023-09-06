use axum::{body::Bytes, extract::Multipart, http::StatusCode, Json};
use twist_drive_core::bytes_hash;

use std::{fs, path::Path};

use crate::{CommonResp, DATA_DIR};
use std::io::prelude::*;

use super::{resp_err, resp_ok};

pub async fn upload_route(files: Multipart) -> (StatusCode, Json<CommonResp>) {
    let (data, name, path, hash) = match parse_input(files).await {
        Ok(x) => x,
        Err(msg) => return resp_err(&msg),
    };
    match name {
        None => resp_err("file field name missing"),
        Some(name) => match data {
            None => resp_err("file field data missing"),
            Some(data) => match path {
                None => resp_err("path field missing"),
                Some(path) => match hash {
                    None => resp_err("hash field missing"),
                    Some(hash) => {
                        if data.is_empty() {
                            return resp_err("file data empty");
                        }
                        if !bytes_hash(&data).eq_ignore_ascii_case(&hash) {
                            return resp_err("file data & hash(md5) not match");
                        }
                        match save_file(&name, &path, &data) {
                            Ok(_) => resp_ok("had saved"),
                            Err(err) => resp_err(&err),
                        }
                    }
                },
            },
        },
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
    ),
    String,
> {
    let mut data: Option<Bytes> = None;
    let mut name: Option<String> = None;
    let mut path: Option<String> = None;
    let mut hash: Option<String> = None; // md5

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
        } else {
            return Err(format!("unknown field:{category}"));
        }
    }
    Ok((data, name, path, hash))
}

fn save_file(name: &str, path: &str, data: &Bytes) -> Result<(), String> {
    let mut dir = DATA_DIR.get().unwrap().clone();
    dir.push('/');
    dir.push_str(path);

    match fs::create_dir_all(Path::new(&dir)) {
        Ok(_) => (),
        Err(err) => return Err(format!("create dir:'{}' fail:{}", dir, err)),
    };

    let file = format!("{}/{}", &dir, name);
    let mut f = match std::fs::File::create(&file) {
        Ok(v) => v,
        Err(err) => return Err(format!("file:'{}' create fail:{}", &file, err)),
    };
    match f.write_all(data) {
        Ok(_) => {}
        Err(err) => return Err(format!(" file:'{}' write data fail:{}", &file, err)),
    };
    match f.flush() {
        Ok(_) => {}
        Err(err) => return Err(format!(" file:'{}' flush data fail:{}", &file, err)),
    };
    Ok(())
}
