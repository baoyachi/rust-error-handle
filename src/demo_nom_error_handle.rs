//1. read file
//2. nom parser
//3. parser time


use std::fs;
use nom::sequence::tuple;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::alphanumeric1;
use time::PrimitiveDateTime;

fn read_file(path: String) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap();
    let mut lines = Vec::new();
    for line in content.lines() {
        if !line.clone().trim().is_empty() {
            lines.push(line.to_string())
        }
    }
    lines
}

fn parser_time(input: &str) -> nom::IResult<&str, (String, String)> {
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

fn get_time(path: String) -> Vec<(String, PrimitiveDateTime)> {
    let mut times = Vec::new();
    for line in read_file(path) {
        let (_, (key, value)) = parser_time(line.as_str()).unwrap();
        let time = PrimitiveDateTime::parse(value, "%F %T").unwrap();
        times.push((key, time))
    }
    times
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Date;

    #[test]
    fn test_parser_time() {
        let input = r#"
        time = "2020/03/01 15:30:22"
        "#;
        let (input, (key, value)) = parser_time(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(key, "time");
        assert_eq!(value, "2020-03-01 15:30:22");
    }

    #[test]
    fn test_convert_time() {
        let path = "./time_tpl.file";
        for (key,value) in get_time(path.to_string()){
            assert_eq!(key,"time");
            assert_eq!(value,PrimitiveDateTime::new(date!(2020-03-01), time!(15:30:22)));
        }

    }
}