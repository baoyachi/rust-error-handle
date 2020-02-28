fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fiel() {
        if let Ok(v) = read_file("./dat") {
            println!("{}", v);
            assert_eq!("8",v)
        }
    }
}

//fn main() {
//    if let Ok(v) = read_file("./dat") {
//        println!("{}", v);
//    }
//}