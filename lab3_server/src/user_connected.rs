use crate::Connection;
use crate::database::Database;
use crate::user::UserAccount;
use std::error::Error;

/// Used to represent a connected user for the actions
pub struct ConnectedUser {
    username: Option<String>,
    pub conn: Connection,
}

impl ConnectedUser {
    pub fn anonymous(conn: Connection) -> ConnectedUser {
        ConnectedUser {
            username: None,
            conn,
        }
    }

    pub fn username(&mut self) -> String {
        self.username.as_ref().unwrap().clone()
    }

    pub fn conn(&mut self) -> &mut Connection {
        &mut self.conn
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = Some(username.to_string());
    }

    pub fn is_anonymous(&self) -> bool {
        return self.username.is_none();
    }

    pub fn logout(&mut self) {
        self.username = None;
    }

    pub fn user_account(&mut self) -> Result<UserAccount, Box<dyn Error>> {
        // No log cause the server crashes if it doesn't work
        Ok(Database::get(&self.username())?.expect("User logged in but not in DB"))
    }
}