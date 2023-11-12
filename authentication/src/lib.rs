use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);

    let users = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("kesh", "password", LoginRole::User)
//     ]
// }

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("kesh".to_string(), User::new("kesh", "password", LoginRole::User));
    users
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role
        }
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("Admin", "password"), LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("admin", "incorrect"), LoginAction::Denied);
        assert_eq!(login("kesh", "password"), LoginAction::Granted(LoginRole::User));
        assert_eq!(login("Kesh", "password"), LoginAction::Granted(LoginRole::User));
        assert_eq!(login("kesh", "incorrect"), LoginAction::Denied);
    }
}
