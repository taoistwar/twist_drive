use crate::{ClientError, Opt};
use anyhow::Context;
use async_recursion::async_recursion;
use log::{debug, error};
use reqwest::{
    multipart::{Form, Part},
    Client, Method,
};
use std::{fs, path::Path};
use twist_drive_core::{file_hash, CommonResp, FileSign};

fn is_default(data: &str) -> bool {
    data.is_empty() || data == "/" || data == r"\"
}

pub async fn upload(args: &Opt) -> Result<(), ClientError> {
    let local_data_dir = Path::new(&args.local_data_dir);
    if !local_data_dir.exists() {
        error!("local data dir: {} not exists", &args.local_data_dir);
        return Ok(());
    }

    let remote_data_dir = if is_default(&args.remote_data_dir) {
        let path = Path::new(&args.local_data_dir)
            .file_name()
            .unwrap()
            .to_string_lossy();
        path.to_string()
    } else {
        args.remote_data_dir.clone()
    };

    if local_data_dir.is_dir() {
        do_upload_dir(&args.server, &args.local_data_dir, &remote_data_dir).await?;
    } else {
        do_upload_file(&args.server, &args.local_data_dir, &remote_data_dir).await?;
    }

    Ok(())
}

#[async_recursion]
async fn do_upload_dir(
    server: &str,
    local_data_dir: &str,
    remote_data_dir: &str,
) -> Result<(), ClientError> {
    for item in fs::read_dir(local_data_dir)? {
        let item = item?.path();
        let local = item.to_string_lossy();

        let remote = format!(
            "{}/{}",
            remote_data_dir,
            item.file_name().unwrap().to_string_lossy()
        );
        debug!("local:{local}");
        debug!("remote:{:?}", remote);
        if item.is_file() {
            do_upload_file(server, &local, &remote).await?;
        } else {
            do_upload_dir(server, &local, &remote).await?;
        }
    }
    Ok(())
}

async fn do_upload_file(
    server: &str,
    local_data_dir: &str,
    remote_data_dir: &str,
) -> Result<(), ClientError> {
    let hash = file_hash(local_data_dir)?;
    let meta = fs::metadata(local_data_dir).context("read meta fail")?;
    let remote_data_dir = if remote_data_dir.is_empty() {
        local_data_dir
    } else {
        remote_data_dir
    };
    debug!("check file hash is exists...");
    if is_exists(&hash, &remote_data_dir, meta.len(), server).await? {
        error!("file exists:{}", remote_data_dir);
        return Ok(());
    }
    debug!("start upload file...");

    let file_name = Path::new(local_data_dir)
        .file_name()
        .ok_or(ClientError::FileName {
            path: local_data_dir.to_string(),
        })?
        .to_str()
        .ok_or(ClientError::FileName {
            path: local_data_dir.to_string(),
        })?
        .to_owned()
        .clone();

    let client = reqwest::Client::builder().build()?;

    let url = format!("http://{}/api/upload", server);
    let data = std::fs::read(local_data_dir.to_string().clone())?;
    let part = Part::bytes(data);
    let part = part.file_name(file_name);
    let form = Form::new()
        .part("file", part)
        .text("path", remote_data_dir.to_string())
        .text("hash", hash.to_string())
        .text("force", "true");

    let request = client.request(Method::POST, url).multipart(form);

    let response = request.send().await?;
    if response.status().as_u16() == 200u16 {
        let body = &response.text().await?;
        let response = serde_json::from_str::<CommonResp>(body)?;
        debug!("upload response: {:?}", &response);
        if !response.status {
            println!("upload fail");
            return Err(ClientError::ActionFail { msg: response.msg });
        }
        return Ok(());
    }

    Ok(())
}

async fn is_exists(
    hash: &str,
    remote_data_dir: &str,
    size: u64,
    server: &str,
) -> Result<bool, ClientError> {
    let client = Client::builder().build()?;
    let url = format!("http://{}/api/exists", server);

    let request = client
        .request(Method::POST, url)
        .json::<FileSign>(&FileSign {
            hash: hash.into(),
            file: remote_data_dir.into(),
            size,
        });

    let response = request.send().await?;
    if response.status().as_u16() == 200u16 {
        let body = &response.text().await?;
        let response = serde_json::from_str::<CommonResp>(body)?;

        debug!("is exists response: {:?}", &response);
        return Ok(response.status);
    }

    Ok(false)
}
