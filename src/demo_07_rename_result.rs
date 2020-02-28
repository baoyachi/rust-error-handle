use std::io::Error as IoError;
use std::str::Utf8Error;
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};

///读取文件内容
fn _read_file(path: &str) -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn _to_utf8(v: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn _to_u32(v: &str) -> std::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

//----Convert Error

#[derive(Debug)]
enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CustomError::IoError(ref e) => Some(e),
            CustomError::Utf8Error(ref e) => Some(e),
            CustomError::ParseIntError(ref e) => Some(e),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(ref e) => e.fmt(f),
            CustomError::Utf8Error(ref e) => e.fmt(f),
            CustomError::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for CustomError {
    fn from(s: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(s)
    }
}

impl From<IoError> for CustomError {
    fn from(s: std::io::Error) -> Self {
        CustomError::IoError(s)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(s: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(s)
    }
}

///自定义Result类型：IResult
pub type IResult<I> = std::result::Result<I, CustomError>;
pub type IOResult<I, O> = std::result::Result<(I, O), CustomError>;

///读取文件内容
fn read_file(path: &str) -> IResult<String> {
    let val = std::fs::read_to_string(path)?;
    Ok(val)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> IResult<&str> {
    let x = std::str::from_utf8(v)?;
    Ok(x)
}

/// 转化为u32数字
fn to_u32(v: &str) -> IResult<u32> {
    let i = v.parse::<u32>()?;
    Ok(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u32() -> std::result::Result<(), CustomError> {
        let path = "./dat";
        let v = read_file(path)?;
        let x = to_utf8(v.as_bytes())?;
        let u = to_u32(x)?;
        assert_eq!(8, u);
        Ok(())
    }

}