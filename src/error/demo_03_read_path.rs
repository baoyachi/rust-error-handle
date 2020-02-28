fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let path = "/tmp/dat";

        ///panic error:
        ///called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        println!("{}", read_file(path));
    }
}