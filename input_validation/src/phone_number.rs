use lazy_static::lazy_static;
use regex::Regex;

// TODO
static REGEX_PHONE_NUMBER: &str = r"[[:upper:]]";

pub fn validate_phone_number(phone_number_input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("{}", REGEX_PHONE_NUMBER)).unwrap();
    }
    RE.is_match(phone_number_input)
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