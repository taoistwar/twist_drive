use crypto::{digest::Digest, md5::Md5};

pub fn file_md5(file: &str) -> String {
    let mut hash = Md5::new();
    use std::io::prelude::*;
    let mut f = std::fs::File::open(file).unwrap();
    let mut buf = vec![0; 1024];
    loop {
        let n = f.read(&mut buf[..]);
        if n.is_err() {
            break;
        }
        let n = n.unwrap();
        let data = &buf[..n];
        hash.input(data);
    }
    hash.result_str()
}
