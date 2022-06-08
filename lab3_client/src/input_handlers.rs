use read_input::prelude::*;
use input_validation::phone_number::validate_phone_number;
use input_validation::password::validate_password;
use input_validation::username::validate_username;
use input_validation::messages::*;

pub fn ask_username() -> String {
    loop {
        let username_input = input::<String>().msg("Please enter the username: ").get();
        if validate_username(&username_input) {
            return username_input;
        }
        println!("{}", INVALID_USERNAME);
    }
}

pub fn ask_password() -> String {
    loop {
        let password_input = input::<String>().msg("Please enter the password: ").get();
        if validate_password(&password_input) {
            return password_input;
        }
        println!("{}", INVALID_PASSWORD.to_string());
    }
}

pub fn ask_phone_number() -> String {
    loop {
        let phone_input = input::<String>().msg("Please enter the phone number: ").get();
        if validate_phone_number(&phone_input) {
            return phone_input;
        }
        println!("{}", INVALID_PHONE_NUMBER.to_string());
    }
}