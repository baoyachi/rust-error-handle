fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let path = "/tmp/dat";
        match read_file(path) {
            Ok(file) => { println!("{}", file) } //OK
            Err(e) => { println!("{} {}", path, e) } //Error:/tmp/dat No such file or directory (os error 2)
        }
    }
}