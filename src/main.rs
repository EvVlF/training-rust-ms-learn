mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password)
        }
    }

    fn hash_password(input: &str) -> u64 { /*...*/ }
}

fn main() {
    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.username);
    println!("The password is: {}", user.get_username());
}