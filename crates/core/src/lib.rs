mod files;
mod logs;

pub use files::*;
pub use logs::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = files::file_hash_sha_256("cargo.toml").unwrap();
        println!("{}", result);
    }
}
