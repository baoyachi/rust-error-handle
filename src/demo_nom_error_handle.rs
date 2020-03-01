//1. read file
//2. nom parser
//3. parser time

use std::fs;
use nom::sequence::tuple;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::alphanumeric1;

fn read_file(path: String) {
    let content = fs::read_to_string(path).unwrap();
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

    //(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str)

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
    println!("{:?}", time.clone());
    Ok((input, (y1.to_string(), time)))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_time() {
        let input = r#"
        time = "2020/03/01 15:30:22"
        "#;
        println!("{:?}", parser_time(input));
    }
}