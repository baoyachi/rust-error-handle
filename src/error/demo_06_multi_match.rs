///读取文件内容
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u32() {
        let path = "./dat";
        match read_file(path) {
            Ok(v) => {
                match to_utf8(v.as_bytes()) {
                    Ok(u) => {
                        match to_u32(u) {
                            Ok(t) => {
                                println!("num:{:?}", u);
                            }
                            Err(e) => {
                                println!("{} {}", path, e)
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} {}", path, e)
                    }
                }
            }
            Err(e) => {
                println!("{} {}", path, e)
            }
        }
    }
}