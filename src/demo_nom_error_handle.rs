use std::fs;
use nom::sequence::tuple;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::alphanumeric1;
use time::PrimitiveDateTime;
use time::ParseError as TimeParseError;
use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::Display;
use nom::error::ParseError;

pub type Result<I> = std::result::Result<I, Error<I>>;
pub type IResult<I, O> = std::result::Result<(I, O), Error<I>>;


#[derive(Debug)]
pub struct Error<I> {
    kind: ErrorKind,
    error: nom::error::VerboseError<I>,
    incomplete: String,

}

impl<I> Error<I> {
    pub fn new(kind: ErrorKind) -> Error<I> {
        Error {
            kind,
            error: nom::error::VerboseError { errors: vec![] },
            incomplete: "".to_string(),
        }
    }

    pub fn new_nom_error(input: I, kind: nom::error::ErrorKind) -> Self {
        Self {
            kind: ErrorKind::NomParserError("Nom parser error".to_string()),
            error: nom::error::VerboseError::from_error_kind(input, kind),
            incomplete: "".to_string(),
        }
    }
}


#[derive(Debug)]
pub enum ErrorKind {
    IoError(IoError),
    TimeParseError(TimeParseError),
    NomParserError(String),
    StringError(String),
    EmptyError,
}

impl<I: std::fmt::Debug> StdError for Error<I> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.kind {
            ErrorKind::IoError(ref e) => Some(e),
            ErrorKind::TimeParseError(ref e) => Some(e),
            ErrorKind::NomParserError(_) => None,
            ErrorKind::StringError(_) => None,
            ErrorKind::EmptyError => None,
        }
    }
}

impl<I> Display for Error<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IoError(ref e) => e.fmt(f),
            ErrorKind::TimeParseError(ref e) => e.fmt(f),
            ErrorKind::NomParserError(ref e) => e.fmt(f),
            ErrorKind::StringError(ref e) => e.fmt(f),
            ErrorKind::EmptyError => write!(f, "Empty Error"),
        }
    }
}

impl<I> From<TimeParseError> for Error<I> {
    fn from(e: TimeParseError) -> Self {
        Error::new(ErrorKind::TimeParseError(e))
    }
}

impl<I> From<std::string::String> for Error<I> {
    fn from(s: std::string::String) -> Self {
        Error::new(ErrorKind::StringError(s))
    }
}


impl<I> From<IoError> for Error<I> {
    fn from(s: IoError) -> Self {
        Error::new(ErrorKind::IoError(s))
    }
}

impl<'a> From<Error<&'a str>> for Error<()> {
    fn from(s: Error<&'a str>) -> Self {
        Error::new(ErrorKind::StringError(format!(
            "kind:{:?},error:{:?}",
            s.kind, s.error
        )))
    }
}


impl<I: std::fmt::Debug> From<nom::Err<(I, nom::error::ErrorKind)>> for Error<I> {
    fn from(i: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        match i {
            nom::Err::Error(err) | nom::Err::Failure(err) => Error::new_nom_error(err.0, err.1),
            nom::Err::Incomplete(i) => Error::new(ErrorKind::StringError(format!(
                "Nom parser Incomplete error: {:?}",
                i
            ))),
        }
    }
}


impl<I> Into<nom::error::VerboseError<I>> for Error<I> {
    fn into(self) -> nom::error::VerboseError<I> {
        self.error
    }
}

impl<I: std::fmt::Debug> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Error::new_nom_error(input, kind)
    }

    fn append(input: I, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other
            .error
            .errors
            .push((input, nom::error::VerboseErrorKind::Nom(kind)));
        other
    }

    fn from_char(input: I, c: char) -> Self {
        Self {
            kind: ErrorKind::EmptyError,
            error: nom::error::VerboseError::from_char(input, c),
            incomplete: "".to_string(),
        }
    }

    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other
            .error
            .errors
            .push((input, nom::error::VerboseErrorKind::Context(ctx)));
        other
    }
}


// nom parser:2020/03/01 15:30:22 -> 2020-03-01 15:30:22
fn parser_time(input: &str) -> IResult<&str, (String, String)> {
    //"2020/03/01 15:30:22" or "  2020/03/01 15:30:22       "
    let time_value = tuple((
        tag("\""),
        multispace0,
        digit1,
        tag("/"),
        digit1,
        tag("/"),
        digit1,
        multispace1,
        digit1,
        tag(":"),
        digit1,
        tag(":"),
        digit1,
        multispace0,
        tag("\""),
        multispace0,
    ));

    //time = "2020-03-01 15:30:22"
    let (input, (_, y1, _, _, _, (_, _, y2, _, y3, _, y4, _, y5, _, y6, _, y7, _, _, _, ))) = tuple((
        multispace0,
        alphanumeric1,
        multispace0,
        tag("="),
        multispace0,
        time_value
    ))(input)?;

    let time = format!("{}-{}-{} {}:{}:{}", y2, y3, y4, y5, y6, y7);
    Ok((input, (y1.to_string(), time)))
}

// parser time
fn get_time<'a>(content: &'a str) -> IResult<&'a str, (String, PrimitiveDateTime)> {
    let (input, (key, value)) = parser_time(content)?;
    let time = PrimitiveDateTime::parse(value, "%F %T")?;
    Ok((input, (key, time)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Date;

    #[test]
    fn test_parser_time() -> Result<()>{
        let input = r#"
        time = "2020/03/01 15:30:22"
        "#;
        let (input, (key, value)) = parser_time(input)?;
        assert_eq!(input, "");
        assert_eq!(key, "time");
        assert_eq!(value, "2020-03-01 15:30:22");
        Ok(())
    }

    #[test]
    fn test_convert_time() -> Result<()>{
        let path = "./time_tpl.file";
        let content = fs::read_to_string(path)?;
        for line in content.clone().lines().into_iter() {
            if line.clone().trim().is_empty() {
                continue;
            }
            let (input, (key, value)) = get_time(line)?;
            assert_eq!(input, "");
            assert_eq!(key, "time");
            assert_eq!(value, PrimitiveDateTime::new(date!(2020-03-01), time!(15:30:22)));
        }
        Ok(())
    }
}