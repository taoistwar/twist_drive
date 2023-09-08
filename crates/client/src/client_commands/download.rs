use std::{
    fs::{self},
    path::Path,
};

use anyhow::Context;
use twist_drive_core::{file_hash, FileSign};

use crate::{ClientError, Opt};

pub async fn download(args: &Opt) -> anyhow::Result<(), ClientError> {
    if Path::new(&args.local_data_dir).exists() {
        let hash = file_hash(&args.local_data_dir);
        let meta = fs::metadata(&args.local_data_dir)
            .with_context(|| format!("get file meta fail: {}", &args.local_data_dir))?;

        let local_data_dir = if args.remote_data_dir.is_empty() {
            args.local_data_dir.clone()
        } else {
            args.remote_data_dir.clone()
        };

        if is_exists(&hash, &local_data_dir, meta.len(), &args.server).await? {
            println!("file exists:{}", &args.remote_data_dir);
            return Ok(());
        }
        println!("local data dir: {} not exists", &args.local_data_dir);
        return Ok(());
    }

    do_download(&args.local_data_dir, &args.remote_data_dir, &args.server).await?;
    Ok(())
}

async fn do_download(
    local_data_dir: &str,
    remote_data_dir: &str,
    server: &str,
) -> anyhow::Result<(), ClientError> {
    let client = reqwest::Client::builder().build()?;

    let remote_data_dir = if let Some(end) = remote_data_dir.strip_prefix('/') {
        end
    } else {
        remote_data_dir
    };

    let url = format!("http://{}/api/download/{}", server, remote_data_dir);
    let request = client.request(reqwest::Method::GET, url);
    let response = request.send().await?;
    let data = response.bytes().await?;
    save_file(local_data_dir, remote_data_dir, &data)?;
    Ok(())
}

async fn is_exists(
    hash: &str,
    remote_data_dir: &str,
    size: u64,
    server: &str,
) -> Result<bool, ClientError> {
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

fn save_file(
    local_data_dir: &str,
    remote_data_dir: &str,
    data: &bytes::Bytes,
) -> anyhow::Result<(), ClientError> {
    let dir = format!("{}/{}", local_data_dir, remote_data_dir);
    let file = Path::new(&dir);

    match file.parent() {
        None => {
            return Err(ClientError::FileParent {
                path: remote_data_dir.to_string(),
            })
        }
        Some(parent) => {
            fs::create_dir_all(parent).context(format!("create dir:'{}'", dir))?;
        }
    }

    let mut f = std::fs::File::create(file)?;
    use std::io::prelude::*;
    f.write_all(data)?;
    f.flush()?;
    Ok(())
}
