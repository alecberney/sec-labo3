use lazy_static::lazy_static;
use regex::Regex;

// TODO
static REGEX_ROLE: &str = r"[[:upper:]]";

pub fn validate_role(role_input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_ROLE)).unwrap();
    }
    RE.is_match(role_input)
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