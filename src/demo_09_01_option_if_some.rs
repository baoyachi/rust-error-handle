fn opt_val(num: i32) -> Option<String> {
    if num >= 60 {
        return Some("foo bar".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_val() {
        if let Some(v) = opt_val(60) {
            println!("{}", v);
            assert_eq!(v,"foo bar".to_string());
        }
    }
}


//fn main() {
//    if let Some(v) = opt_val(60) {
//        println!("{}", v);
//    }
//}