/// This file is used to store and retrieve user accounts from the database
///
/// Tasks: - Log stuff whenever required
///        - Potential improvements
use crate::user::{UserAccount, UserRole};
use crate::hashing_tools::new_hash_password;
use crate::env_reader::read_env_file;
use lazy_static::lazy_static;
use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use log::info;

lazy_static! {
    static ref DB: FileDatabase<Database, Ron> =
        FileDatabase::load_from_path_or_default("db.ron").unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Database {
    data: HashMap<String, UserAccount>,
}

impl Database {
    pub fn insert(user: &UserAccount) -> Result<(), Box<dyn Error>> {
        DB.write(|db| db.data.insert(user.username().to_string(), user.clone()))?;
        Ok(DB.save()?)
    }

    pub fn get(username: &str) -> Result<Option<UserAccount>, Box<dyn Error>> {
        Ok(match DB.borrow_data()?.data.get(username) {
            Some(user) => Some(user.clone()),
            None => None,
        })
    }

    pub fn values() -> Result<Vec<UserAccount>, Box<dyn Error>> {
        Ok(DB.borrow_data()?.data.values().cloned().collect())
    }

    pub fn init() {
        // Do nothing but awake the lazy_static
        let _ = &DB.borrow_data();
    }
}

impl Default for Database {
    fn default() -> Self {
        let mut db = Database {
            data: HashMap::new(),
        };

        info!("Creating starting data for database");

        // Reads env file
        let config = match read_env_file() {
            Ok(config) => config,
            Err(e) => panic!("An error occurred reading env file: {}", e)
        };

        let (default_salt_user, default_hash_pwd_user)
            = new_hash_password(&config.default_user_password);
        let (default_salt_hr, default_hash_pwd_hr)
            = new_hash_password(&config.default_hr_password);

        let user = UserAccount::new(
            config.default_user,
            default_hash_pwd_user,
            default_salt_user,
            config.default_user_phone,
            UserRole::StandardUser,
        );

        let hr = UserAccount::new(
            config.default_hr,
            default_hash_pwd_hr,
            default_salt_hr,
            config.default_hr_phone,
            UserRole::HR,
        );

        db.data.insert(user.username().to_string(), user);
        db.data.insert(hr.username().to_string(), hr);

        db
    }
}
