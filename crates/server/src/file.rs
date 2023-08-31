use axum::{extract::Multipart, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

// handler to upload image or file
pub async fn upload_route(
    mut files: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut res = HashMap::new();
    while let Some(file) = files.next_field().await.unwrap() {
        // this is the name which is sent in formdata from frontend or whoever called the api, i am
        // using it as category, we can get the filename from file data
        let category = file.name().unwrap().to_string();
        // name of the file with extention
        let name = file.file_name().unwrap().to_string();
        // file data
        let data = file.bytes().await.unwrap();
        // the path of file to store on aws s3 with file name and extention
        // timestamp_category_filename => 14-12-2022_01:01:01_customer_somecustomer.jpg
        let key = format!(
            "images/{}_{}_{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            &category,
            &name
        );

        dbg!(&key);
        res.insert(
            // concatinating name and category so even if the filenames are same it will not
            // conflict
            format!("{}_{}", &name, &category),
            format!("{}/{}", &key, data.len()),
        );
    }
    // send the urls in response
    Ok(Json(serde_json::json!(res)))
}

pub async fn exists_route(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<FileSign>,
) -> (StatusCode, Json<FileResp>) {
    // insert your application logic here

    let file = payload.file;
    let path = Path::new(&file);
    // 存在
    if !path.exists() {
        return resp_msg(false, "file not exists");
    }
    if let Ok(meta) = fs::metadata(path) {
        // 类型=file
        if !meta.is_file() {
            return resp_msg(false, "server path is dir");
        }
        // 大小
        if meta.len() != payload.size && !meta.is_file() {
            return resp_msg(false, "file size not match, need reupload");
        }
    }
    // md5相同
    let md5 = twist_core::file_md5(&file);
    if md5 != payload.sign {
        return resp_msg(false, "file sign not match, need reupload");
    }

    resp_msg(true, "file had uploaded")
}

fn resp_msg(status: bool, msg: &str) -> (StatusCode, Json<FileResp>) {
    (
        StatusCode::OK,
        Json(FileResp {
            status,
            msg: msg.into(),
        }),
    )
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct FileSign {
    sign: String,
    file: String,
    size: u64,
}
// the output to our `create_user` handler
#[derive(Serialize)]
pub struct FileResp {
    status: bool,
    msg: String,
}
