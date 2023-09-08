use anyhow::Context;
use log::{debug, error};
use reqwest::{
    multipart::{Form, Part},
    Client, Method,
};
use std::{fs, path::Path};
use twist_drive_core::{file_hash, get_file_name, CommonResp, FileSign};

use crate::{ClientError, Opt};

pub async fn upload(args: &Opt) -> Result<(), ClientError> {
    if !Path::new(&args.local_data_dir).exists() {
        error!("local data dir: {} not exists", &args.local_data_dir);
        return Ok(());
    }

    let hash = file_hash(&args.local_data_dir);
    let meta = fs::metadata(&args.local_data_dir).context("read meta fail")?;
    let remote_data_dir = if args.remote_data_dir.is_empty() {
        args.local_data_dir.clone()
    } else {
        args.remote_data_dir.clone()
    };
    debug!("check file hash is exists...");
    if is_exists(&hash, &remote_data_dir, meta.len(), &args.server).await? {
        error!("file exists:{}", &args.remote_data_dir);
        return Ok(());
    }
    debug!("start upload file...");
    let file_name = get_file_name(&args.local_data_dir).ok_or(ClientError::FileName {
        path: args.local_data_dir.to_string(),
    })?;
    do_upload(
        &args.server,
        &args.local_data_dir,
        &args.remote_data_dir,
        &hash,
        file_name.to_string(),
    )
    .await?;

    Ok(())
}

async fn do_upload(
    server: &str,
    local_data_dir: &str,
    remote_data_dir: &str,
    hash: &str,
    file_name: String,
) -> Result<(), ClientError> {
    let client = reqwest::Client::builder().build()?;

    let local_data_dir: String = local_data_dir.to_string().clone();

    let url = format!("http://{}/api/upload", server);
    let data: Vec<u8> = std::fs::read(local_data_dir)?;
    let part = Part::bytes(data);
    let part = part.file_name(file_name);
    let form = Form::new()
        .part("file", part)
        .text("path", remote_data_dir.to_string())
        .text("hash", hash.to_string());

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
