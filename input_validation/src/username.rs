use lazy_static::lazy_static;
use regex::Regex;

// TODO
static REGEX_USERNAME: &str = r"[[:upper:]]";

pub fn validate_username(username_input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_USERNAME)).unwrap();
    }
    RE.is_match(username_input)
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}