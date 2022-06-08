extern crate envfile;

use envfile::EnvFile;
use std::path::Path;
use std::error::Error;

// TODO: To use this server, create a .env file at the root and add these values to it:
//SERVER_IP=
//KEY_PATH=
//CERT_PATH=
//DEFAULT_USER=
//DEFAULT_USER_PASSWORD=
//DEFAULT_USER_PHONE=
//DEFAULT_HR=
//DEFAULT_HR_PASSWORD=
//DEFAULT_HR_PHONE=

pub struct Config {
    pub server_ip: String,
    pub key_path: String,
    pub certificate_path: String,
    pub default_user: String,
    pub default_user_password: String,
    pub default_user_phone: String,
    pub default_hr: String,
    pub default_hr_password: String,
    pub default_hr_phone: String,
}

pub fn read_env_file() -> Result<Config, Box<dyn Error>> {
    let envfile = EnvFile::new(&Path::new(".env"))?;

    let mut config = Config {
        server_ip: "".to_string(),
        key_path: "".to_string(),
        certificate_path: "".to_string(),
        default_user: "".to_string(),
        default_user_password: "".to_string(),
        default_user_phone: "".to_string(),
        default_hr: "".to_string(),
        default_hr_password: "".to_string(),
        default_hr_phone: "".to_string()
    };

    for (key, value) in envfile.store {
        match &*key {
            "SERVER_IP" => config.server_ip = format!("{}", value),
            "KEY_PATH" => config.key_path = format!("{}", value),
            "CERT_PATH" => config.certificate_path = format!("{}", value),
            "DEFAULT_USER" => config.default_user = format!("{}", value),
            "DEFAULT_USER_PASSWORD" => config.default_user_password = format!("{}", value),
            "DEFAULT_USER_PHONE" => config.default_user_phone = format!("{}", value),
            "DEFAULT_HR" => config.default_hr = format!("{}", value),
            "DEFAULT_HR_PASSWORD" => config.default_hr_password = format!("{}", value),
            "DEFAULT_HR_PHONE" => config.default_hr_phone = format!("{}", value),
            _ => {}
        }
    }

    if config.server_ip == ""
        || config.key_path == ""
        || config.certificate_path == ""
        || config.default_user == ""
        || config.default_user_password == ""
        || config.default_user_phone == ""
        || config.default_hr == ""
        || config.default_hr_password == ""
        || config.default_hr_phone == "" {
        Err("Invalid env file".into())
    } else {
        Ok(config)
    }
}