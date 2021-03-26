#[derive(Debug)]
struct User {
    username: String,
    email: String,
    logged_in_count: u64,
    active: bool,
}

fn main() {
    let user = User{
        username: String::from("Aaryan"),
        email: String::from("abc@adcd.com"),
        logged_in_count: 0,
        active: true,
    };
    println!("{:?}", user);
}

