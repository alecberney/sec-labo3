/// This file is used to store and retrieve user accounts from the database
///
/// Tasks: - Potential improvements
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserRole {
    StandardUser,
    HR,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAccount {
    username: String,
    hash_password: String,
    salt: [u8;16],
    phone_number: String,
    role: UserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAccountPublic {
    pub username: String,
    pub phone_number: String,
}

impl UserAccount {
    pub fn new(username: String, hash_password: String, salt: [u8;16], phone_number: String, role: UserRole) -> Self {
        Self {
            username,
            hash_password,
            salt,
            phone_number,
            role,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn hash_password(&self) -> &str {
        &self.hash_password
    }

    pub fn salt(&self) -> &[u8;16] {
        &self.salt
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }
}
