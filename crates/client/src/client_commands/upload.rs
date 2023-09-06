use anyhow::Context;
use reqwest::{
    multipart::{Form, Part},
    Method,
};
use std::{fs, path::Path};
use twist_drive_core::{file_hash, get_file_name, FileSign};

use crate::{ClientError, Opt};

pub async fn upload(args: &Opt) -> anyhow::Result<()> {
    if !Path::new(&args.local_data_dir).exists() {
        println!("local data dir: {} not exists", &args.local_data_dir);
        return Ok(());
    }

    let md5 = file_hash(&args.local_data_dir);

    let meta = fs::metadata(&args.local_data_dir).context("read meta fail")?;

    let remote_data_dir = if args.remote_data_dir.is_empty() {
        args.local_data_dir.clone()
    } else {
        args.remote_data_dir.clone()
    };

    if is_exists(&md5, &remote_data_dir, meta.len(), &args.server).await? {
        println!("file exists:{}", &args.remote_data_dir);
        return Ok(());
    }
    let file_name = get_file_name(&args.local_data_dir).ok_or(ClientError::FileName {
        path: args.local_data_dir.to_string(),
    })?;
    do_upload(
        &args.server,
        &args.local_data_dir,
        &args.remote_data_dir,
        &md5,
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
) -> anyhow::Result<()> {
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
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn is_exists(
    hash: &str,
    remote_data_dir: &str,
    size: u64,
    server: &str,
) -> anyhow::Result<bool> {
    let client = reqwest::Client::builder().build()?;
    let sign = FileSign {
        hash: hash.into(),
        file: remote_data_dir.into(),
        size,
    };
    let url = format!("http://{}/api/exists", server);

    let data = serde_json::to_string(&sign).unwrap();

    let request = client.request(reqwest::Method::POST, url).json(&data);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(false)
}
