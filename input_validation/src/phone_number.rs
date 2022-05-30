use lazy_static::lazy_static;
use regex::Regex;

// We accept only swiss phone number
// We accept only these formats:
// 011 222 33 44
// 00 11 222 33 44
// +41 11 222 33 44
// 0112223344
// 00112223344
// +41112223344

//static REGEX_PHONE_NUMBER: &str = r"((0\d{2})|((00|\+)\d{2} ?))\d{3} ?\d{2} ?\d{2}";
static REGEX_PHONE_NUMBER_FORMAT_1: &str = r"0\d{2} ?\d{3} ?\d{2} ?\d{2}";
static REGEX_PHONE_NUMBER_FORMAT_2: &str = r"\+41 ?\d{2} ?\d{3} ?\d{2} ?\d{2}";
static REGEX_PHONE_NUMBER_FORMAT_3: &str = r"00 ?\d{2} ?\d{3} ?\d{2} ?\d{2}";

// No semantical validation will be done
pub fn validate_phone_number(phone_number_input: &str) -> bool {
    lazy_static! {
        static ref RE_FORMAT_1: Regex = Regex::new(&format!("^{}$", REGEX_PHONE_NUMBER_FORMAT_1)).unwrap();
        static ref RE_FORMAT_2: Regex = Regex::new(&format!("^{}$", REGEX_PHONE_NUMBER_FORMAT_2)).unwrap();
        static ref RE_FORMAT_3: Regex = Regex::new(&format!("^{}$", REGEX_PHONE_NUMBER_FORMAT_3)).unwrap();
    }
    RE_FORMAT_1.is_match(phone_number_input)
    || RE_FORMAT_2.is_match(phone_number_input)
    || RE_FORMAT_3.is_match(phone_number_input)
}

// TODO
#[cfg(test)]
mod tests {
    use super::validate_phone_number;

    #[test]
    fn validate_phone_number_test() {
        assert!(validate_phone_number("078 837 77 18"));
        assert!(validate_phone_number("00 78 837 77 18"));
        assert!(validate_phone_number("+41 78 837 77 18"));
        assert!(validate_phone_number("0788377718"));
        assert!(validate_phone_number("00788377718"));
        assert!(validate_phone_number("+41788377718"));
    }
}