use casbin::CoreApi;
use casbin::prelude::Enforcer;
use std::error::Error;
use log::warn;
use crate::{Action, ConnectedUser, UserRole};

// A Role-Based Access Control (RBAC) system will be implemented
// Here a the roles and the actions that they can perform
//                      |show users|change own phone|change phone|add user|login|logout|exit|
// anonymous users:     |     x    |                |            |        |  x  |      |  x |
// authenticated users: |     x    |        x       |            |        |     |   x  |  x |
// HR users:            |     x    |        x       |      x     |    x   |     |   x  |  x |

const ACCESS_CONTROL_CONF_FILE: &str = "./access_control.conf";
const ACCESS_CONTROL_CSV_FILE: &str = "./access_control.csv";

#[tokio::main]
pub async fn can_perform_action(action: Action, user: &mut ConnectedUser) -> Result<bool, Box<dyn Error>> {
    let mut e = Enforcer::new(
        ACCESS_CONTROL_CONF_FILE,
        ACCESS_CONTROL_CSV_FILE).await?;
    e.enable_log(true);

    match e.enforce((
        get_user_role_string(user)?,
        get_action_string(&action),
    )) {
        Ok(true) => Ok(true),
        Ok(false) => {
            warn!("A user tried to access control an unauthorized ressource {:?}", action);
            Ok(false)
        },
        Err(_) => {
            warn!("Access control error");
            Err("Error while checking access control".into())
        },
    }
}

fn get_action_string(action: &Action) -> &'static str {
    match action {
        Action::ShowUsers => "show_users",
        Action::ChangeOwnPhone => "change_own_phone",
        Action::ChangePhone => "change_phone",
        Action::AddUser => "add_user",
        Action::Login => "login",
        Action::Logout => "logout",
        Action::Exit => "exit",
    }
}

fn get_user_role_string(user: &mut ConnectedUser) -> Result<&str, Box<dyn Error>> {
    if user.is_anonymous() {
        return Ok("anonymous");
    }
    match user.user_account()?.role() {
        UserRole::StandardUser => Ok("normal"),
        UserRole::HR => Ok("hr")
    }
}