use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsStr;

/// 获取二进制的hash码（sha-256），功能和sha256sum类似。
pub fn bytes_hash(data: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(data);
    let res = hash.finalize();

    base16ct::lower::encode_string(&res)
}

/// 获取文件的hash码（sha-256），功能和sha256sum类似。
pub fn file_hash(file: &str) -> String {
    let mut hash = Sha256::new();
    use std::io::prelude::*;
    let mut f = std::fs::File::open(file).unwrap();
    let mut buf = vec![0; 1024];
    loop {
        let n = f.read(&mut buf[..]);
        if n.is_err() {
            break;
        }
        let n = n.unwrap();
        if n == 0 {
            break;
        }
        let data = &buf[..n];
        hash.update(data);
    }
    let res = hash.finalize();

    base16ct::lower::encode_string(&res)
}

/// 获取文件名称，
/// # 示例
/// ```
/// use twist_drive_core::get_file_name;
/// assert_eq!(get_file_name("./create/core/src/file.rs"), Some("file.rs"));
/// assert_eq!(get_file_name("file.rs"), Some("file.rs"));
/// assert_eq!(get_file_name("./file.rs"), Some("file.rs"));
/// ```
pub fn get_file_name(file: &str) -> Option<&str> {
    let path = Path::new(file);
    path.file_name()?.to_str()
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn get_file_dir(path: &str) -> String {
    path.into()
}

///
#[derive(Serialize, Deserialize, Debug)]
pub struct FileSign {
    pub hash: String,
    pub file: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePath {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResp {
    pub status: bool,
    pub msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    static FULL_PATH: &str = "./create/core/src/file.rs";

    #[test]
    fn test_file_name() {
        assert_eq!(get_file_name(FULL_PATH), Some("file.rs"));
        assert_eq!(get_file_name("file.rs"), Some("file.rs"));
        assert_eq!(get_file_name("./file.rs"), Some("file.rs"));
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(get_file_extension(FULL_PATH), Some("rs"));
    }

    #[test]
    fn test_bytes_hash() {
        let bytes = b"example byte string!";
        let hash = bytes_hash(bytes);
        assert_eq!(
            hash,
            "9c1ea8a05f5ad0373a804afe899fbb3e90ddf3d8ffe307992f66150f371df552"
        );
    }

    #[test]
    fn test_file_hash() {
        let hash = file_hash("/mnt/e/app/vam/VaM.exe");
        assert_eq!(
            hash,
            "9f5959a81214322c8246d4915308bceb06ad23f9675b20b9e88b39e028a93bfd"
        );
    }
    #[test]
    fn test_empty_hash() {
        let hash = Sha256::new();
        let res = hash.finalize();

        println!("{}", base16ct::lower::encode_string(&res));
    }
}
