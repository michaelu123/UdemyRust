#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

fn build_admin(username: String, password: String) -> User {
    User {
        is_admin: true,
        username,
        password
    }
}

fn build_user(username: &str, password: &str) -> User {
    User {
        is_admin: false,
        username: String::from(username),
        password: String::from(password),
    }
}

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("Ubootkämpfer3"),
    };

    println!("u1{:?}", user1);
    println!("{}", user1.is_admin);
    println!("{}", user1.username);
    println!("{}", user1.password);

    let user2 = build_admin(String::from("Peter"), String::from("P4SSW0RD"));

    println!("u2{:?}", user2);
    println!("{}", user2.is_admin);
    println!("{}", user2.username);
    println!("{}", user2.password);

    let user3 = build_user("Laura", "PA55W0RD");

    println!("u3{:?}", user3);
    println!("{}", user3.is_admin);
    println!("{}", user3.username);
    println!("{}", user3.password);

}
