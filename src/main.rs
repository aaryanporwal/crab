#[derive(Debug)] // For easily printing our struct
struct User {
    username: String,
    email: String,
    logged_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        logged_in_count: 0,
        active: true
    }
}

fn main() {
    let user = build_user(String::from("Aaryan"), String::from("aaryan@aaryan.com"));
    
    println!("{:?}", user);
}

