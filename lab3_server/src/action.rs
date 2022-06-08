/// This file is used to execute the various actions submitted by the clients
///
/// Tasks: - Improve the authentication & access controls
///        - Input/output validation
///        - Log stuff whenever required
///        - Potential improvements
use crate::database::Database;
use crate::user::{UserAccount, UserAccountPublic, UserRole};
use crate::user_connected::ConnectedUser;
use crate::messages::*;
use crate::hashing_tools::*;
use crate::access_control::can_perform_action;
use serde::{Deserialize, Serialize};
use std::error::Error;
use strum_macros::{EnumIter, EnumString};
use log::{info, trace, warn};
use input_validation::phone_number::validate_phone_number;
use input_validation::password::validate_password;
use input_validation::username::validate_username;
use input_validation::messages::*;

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
        trace!("Performing action: {:?}", self);

        let res = match self {
            Action::ShowUsers => Action::show_users(u),
            Action::ChangeOwnPhone => Action::change_own_phone(u),
            Action::ChangePhone => Action::change_phone(u),
            Action::AddUser => Action::add_user(u),
            Action::Login => Action::login(u),
            Action::Logout => Action::logout(u),
            Action::Exit => {
                info!("Client disconnected");
                Err("Client disconnected")?
            },
        };

        res
    }

    pub fn show_users(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Show users");

        // Check permissions
        let res = if can_perform_action(Action::ShowUsers, u)? {
            // Update phone number
            let users = Database::values()?;
            let mut users_public: Vec<UserAccountPublic> = vec![];
            for user in users {
                users_public.push(UserAccountPublic {
                    username: user.username().to_string(),
                    phone_number: user.phone_number().to_string()
                });
            }
            Ok(users_public)
        } else {
            warn!("Someone tried to see users");
            Err(PERMISSION_DENIED)
        };

        //let res: Result<Vec<UserAccountPublic>, &str> = Ok(users_public);
        u.conn().send(&res)
    }

    pub fn change_own_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Change own phone number");

        // TODO: ask to reauthenticate

        let phone = u.conn().receive::<String>()?;

        let res;

        // Validate data
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            warn!("Invalid phone number: {}", phone);
            return u.conn().send(&res);
        }

        // Check permissions
        res = if can_perform_action(Action::ChangeOwnPhone, u)? {
            // Update phone number
            let mut user = u.user_account()?;
            user.set_phone_number(phone);
            Database::insert(&user)?;
            info!("Phone number changed");
            Ok(())
        } else {
            warn!("Anonymous user tried to change own phone number");
            Err(PERMISSION_DENIED)
        };

        u.conn().send(&res)
    }

    pub fn change_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Change phone number");

        // TODO: ask to reauthenticate

        // Receive data
        let username = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;
        let target_user = Database::get(&username)?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            warn!("Invalid username: {}", username);
            return u.conn().send(&res);
        }
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            warn!("Invalid phone number: {}", phone);
            return u.conn().send(&res);
        }

        // Check permissions
        res = if can_perform_action(Action::ChangePhone, u)? {
            if target_user.is_none() {
                warn!("User not found: {}", username);
                Err(USER_NOT_FOUND)
            } else {
                // Update phone number from target user
                let mut target_user = target_user.unwrap();
                target_user.set_phone_number(phone);
                Database::insert(&target_user)?;
                info!("Phone number changed for user: {}", username);
                Ok(())
            }
        } else {
            warn!("A user tried to change phone number of user: {}", username);
            Err(PERMISSION_DENIED)
        };

        u.conn().send(&res)
    }

    pub fn add_user(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Adding user");

        // TODO: ask to reauthenticate

        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;
        let role = u.conn().receive::<UserRole>()?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            warn!("Invalid username: {}", username);
            return u.conn().send(&res);
        }
        if !validate_password(&password) {
            res = Err(INVALID_PASSWORD);
            warn!("Invalid password: {}", password);
            return u.conn().send(&res);
        }
        if !validate_phone_number(&phone) {
            res = Err(INVALID_PHONE_NUMBER);
            warn!("Invalid phone number: {}", phone);
            return u.conn().send(&res);
        }
        // Role is validated and can't be false
        // because connection receive will return and throw an error before.

        // Hash password with a random salt
        let (salt, hash_password) = new_hash_password(&password);

        // Check permissions
        res = if can_perform_action(Action::AddUser, u)? {
            if Database::get(&username)?.is_some() {
                warn!("User already exists: {}", username);
                Err(USER_EXISTS)
            } else {
                let user = UserAccount::new(username, hash_password, salt, phone, role);
                info!("User added");
                Ok(Database::insert(&user)?)
            }
        } else {
            warn!("A user tried to add user {}", username);
            Err(PERMISSION_DENIED)
        };

        u.conn.send(&res)
    }

    pub fn login(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Login");

        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;

        let res;

        // Validate data
        if !validate_username(&username) {
            res = Err(INVALID_USERNAME);
            warn!("Invalid username: {}", username);
            return u.conn().send(&res);
        }
        if !validate_password(&password) {
            res = Err(INVALID_PASSWORD);
            warn!("Invalid password: {}", password);
            return u.conn().send(&res);
        }

        // Check permissions
        res = if can_perform_action(Action::Login, u)? {
            let user = Database::get(&username)?;
            if let Some(user) = user {
                // TODO: faire le hash de toute façon
                // Compare hash of passwords
                if user.hash_password() == &hash_argon2(&password, user.salt()) {
                    u.set_username(&username);
                    info!("User {} logged in", username);
                    Ok(())
                } else {
                    warn!("Invalid password for user {}", username);
                    Err(LOGIN_FAIL)
                }
            } else {
                warn!("User not found: {}", username);
                Err(LOGIN_FAIL)
            }
        } else {
            warn!("User {} tried to login", u.username());
            Err(PERMISSION_DENIED)
        };

        u.conn.send(&res)
    }

    pub fn logout(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Logout");

        let res: Result<(), &str>;

        res = if can_perform_action(Action::Logout, u)? {
            // Logout
            info!("User {} logged out", u.username());
            u.logout();
            Ok(())
        } else {
            warn!("Anonymous tried to logout");
            Err(PERMISSION_DENIED)
        };

        u.conn.send(&res)
    }
}