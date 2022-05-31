/// This file is used to execute the various actions submitted by the clients
///
/// Tasks todo: - Improve the authentication & access controls
///             - Input/output validation
///             - Log stuff whenever required
///             - Potential improvements
use crate::connection::Connection;
use crate::database::Database;
use crate::user::{UserAccount, UserRole};
use crate::messages::*;
use crate::hashing_tools::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use strum_macros::{EnumIter, EnumString};
use input_validation::phone_number::validate_phone_number;
use input_validation::password::validate_password;
use input_validation::username::validate_username;

#[derive(Serialize, Deserialize, Debug, EnumString, EnumIter)]
pub enum Action {
    #[strum(serialize = "Show users", serialize = "1")]
    ShowUsers,
    #[strum(serialize = "Change my phone number", serialize = "2")]
    ChangeOwnPhone,
    #[strum(serialize = "Show someone's phone number", serialize = "3")]
    ChangePhone,
    #[strum(serialize = "Add user", serialize = "4")]
    AddUser,
    #[strum(serialize = "Login", serialize = "5")]
    Login,
    #[strum(serialize = "Logout", serialize = "6")]
    Logout,
    #[strum(serialize = "Exit", serialize = "7")]
    Exit,
}

/// The individual actions are implemented with three main steps:
///     1. Read client inputs if required
///     2. Execute various server code
///     3. Send a result
impl Action {
    pub fn perform(&self, u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        let res = match self {
            Action::ShowUsers => Action::show_users(u),
            Action::ChangeOwnPhone => Action::change_own_phone(u),
            Action::ChangePhone => Action::change_phone(u),
            Action::AddUser => Action::add_user(u),
            Action::Login => Action::login(u),
            Action::Logout => Action::logout(u),
            Action::Exit => Err("Client disconnected")?,
        };

        res
    }

    pub fn show_users(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        let users = Database::values()?;
        let res: Result<Vec<UserAccount>, &str> = Ok(users);
        u.conn().send(&res)
    }

    pub fn change_own_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        let phone = u.conn().receive::<String>()?;

        let res;

        // Validate data
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            return u.conn().send(&res);
        }

        // TODO move access validation in an other part
        // Check permissions
        res = if u.is_anonymous() {
            Err(ANONYMOUS_PHONE)
        } else {
            let mut user = u.user_account()?;
            user.set_phone_number(phone);
            Database::insert(&user)?;
            Ok(())
        };

        u.conn().send(&res)
    }

    pub fn change_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        // Receive data
        let username = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;
        let target_user = Database::get(&username)?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            return u.conn().send(&res);
        }
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            return u.conn().send(&res);
        }

        // TODO move access validation in an other part
        // Check permissions
        res = if u.is_anonymous() {
            Err(ANONYMOUS_PHONE)
        } else if let UserRole::StandardUser = u.user_account()?.role() {
            Err(STANDARD_OTHER_PHONE)
        } else if target_user.is_none() {
            Err(USER_NOT_FOUND)
        } else {
            let mut target_user = target_user.unwrap();
            target_user.set_phone_number(phone);
            Database::insert(&target_user)?;
            Ok(())
        };

        u.conn().send(&res)
    }

    pub fn add_user(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;
        let role = u.conn().receive::<UserRole>()?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            return u.conn().send(&res);
        }
        if !validate_password(&password) {
            res = Err(INVALID_PASSWORD);
            return u.conn().send(&res);
        }
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            return u.conn().send(&res);
        }

        // Hash password with a random salt
        let (salt, hash_password) = new_hash_password(&password);

        // TODO move access validation in an other part
        // Check permissions
        res = if u.is_anonymous() {
            Err(ANONYMOUS_ADD_USER)
        } else if let UserRole::HR = u.user_account()?.role() {
            if Database::get(&username)?.is_some() {
                Err(USER_EXISTS)
            } else {
                let user = UserAccount::new(username, hash_password, salt, phone, role);
                Ok(Database::insert(&user)?)
            }
        } else {
            Err(ONLY_HR_ADD_USER)
        };

        u.conn.send(&res)
    }

    pub fn login(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            return u.conn().send(&res);
        }
        if !validate_password(&password) {
            res = Err(INVALID_PASSWORD);
            return u.conn().send(&res);
        }

        // TODO compare hash of pwd and not pwd
        // Check permissions
        res = if !u.is_anonymous() {
            Err(ALREADY_LOGGED_IN)
        } else {
            let user = Database::get(&username)?;
            if let Some(user) = user {
                // Compare hash of passwords
                if user.hash_password() == &hash_argon2(&password, user.salt()) {
                    u.set_username(&username);
                    Ok(())
                } else {
                    Err(INVALID_PASSWORD)
                }
            } else {
                Err(USER_DOES_NOT_EXIST)
            }
        };

        u.conn.send(&res)
    }

    pub fn logout(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        let res: Result<(), &str>;

        // TODO move access validation in an other part
        // Check permissions
        res = if u.is_anonymous() {
            Err(NOT_LOGGED_IN)
        } else {
            u.logout();
            Ok(())
        };

        u.conn.send(&res)
    }
}

// TODO
// The access controls are currently split into multiple functions which makes it hard to maintain.
// Try to improve the access controls with what you saw in the courses
// TODO: move in an other file

/// Used to represent a connected user for the actions
pub struct ConnectedUser {
    username: Option<String>,
    conn: Connection,
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
        Ok(Database::get(&self.username())?.expect("User logged in but not in DB"))
    }
}
