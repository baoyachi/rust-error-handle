fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let path = "/tmp/dat";
        //called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        //println!("{}", read_file(path));
    }
}

//这是一个示例项目，使用`unit test`替代`main`函数执行
//fn main() {
//    let path = "/tmp/dat";
//    println!("{}", read_file(path));
//}