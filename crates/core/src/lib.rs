mod file;

pub use file::*;

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
