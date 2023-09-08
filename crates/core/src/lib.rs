mod file;
mod logs;

pub use file::*;
pub use logs::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = file::file_hash("cargo.toml");
        println!("{}", result);
    }
}
