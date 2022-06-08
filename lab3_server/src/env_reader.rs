extern crate envfile;

use envfile::EnvFile;
use std::path::Path;
use std::error::Error;

// TODO: To use this server, create a .env file at the root and add these values to it:
//SERVER_IP=
//KEY_PATH=
//CERT_PATH=
//DEFAULT_USER=
//DEFAULT_HR=
//DEFAULT_PASSWORD=d

/*let mut smtp_user;
let mut smtp_pass;
let mut smtp_serv;
let mut mail_from;

fn read_env_file() -> Result<(String, String, String, String), Box<dyn Error>> {
    let envfile = EnvFile::new(&Path::new("./.env"))?;

    let mut smtp_user= String::from("");
    let mut smtp_pass= String::from("");
    let mut smtp_serv= String::from("");
    let mut mail_from= String::from("");

    for (key, value) in envfile.store {
        match &*key {
            "SMTP_USER" => smtp_user = format!("{}", value),
            "SMTP_PASS" => smtp_pass = format!("{}", value),
            "SMTP_SERV" => smtp_serv = format!("{}", value),
            "MAIL_FROM" => mail_from = format!("{}", value),
            _ => {}
        }
    }

    if smtp_user == "" || smtp_pass == "" || smtp_serv == "" || mail_from == "" {
        Err("INVALID ENV FILE".into())
    } else {
        Ok((smtp_user, smtp_pass, smtp_serv, mail_from))
    }
}*/