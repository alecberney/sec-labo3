/// This file is used to store and retrieve user accounts from the database
///
/// Tasks todo: - Log stuff whenever required
///             - Potential improvements
use crate::user::{UserAccount, UserRole};
use crate::hashing_tools::new_hash_password;
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

        // TODO: secret in env file
        let (default_salt1, default_hash_pwd1) = new_hash_password("default_pass");
        let default_salt2 = default_salt1.clone();
        let default_hash_pwd2 = default_hash_pwd1.clone();

        let u1 = UserAccount::new(
            "default_user".to_string(),
            default_hash_pwd1,
            default_salt1,
            "0784539872".to_string(),
            UserRole::StandardUser,
        );

        let u2 = UserAccount::new(
            "default_hr".to_string(),
            default_hash_pwd2,
            default_salt2,
            "0793175289".to_string(),
            UserRole::HR,
        );

        db.data.insert(u1.username().to_string(), u1);
        db.data.insert(u2.username().to_string(), u2);

        db
    }
}
