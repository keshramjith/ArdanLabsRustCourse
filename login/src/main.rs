use authentication::{login, read_line, LoginAction, LoginRole, User};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username");
        let username = read_line();
        println!("Enter your password");
        let password = read_line();

        let user: User;

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => user = User::new(&username, &password, LoginRole::Admin),
                    LoginRole::User => user = User::new(&username, &password, LoginRole::User)
                }
                break;
            },
            Some(LoginAction::Denied) => {
                tries += 1;
                println!("Invalid username or password");
                if tries >= 3 {
                    println!("Too many failed attempts");
                    break;
                }
            },
            None => {
                println!("Invalid username or password");
                break;
            }
        }
    }
}
