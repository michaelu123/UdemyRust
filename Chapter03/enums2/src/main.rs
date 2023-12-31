#[derive(Debug)]
enum LoginData {
    None,
    Invalid,
    NotRegistered,
    Username(String),
    Other(i8)
}

fn main() {
    let none_user = LoginData::None;
    println!("{:?}", none_user);

    let admin = LoginData::Username(String::from("franneck94"));
    println!("{:?}", admin);

    let inv = LoginData::Invalid;
    println!("{:?}", inv);

    let nr = LoginData::NotRegistered;
    println!("{:?}", nr);

    let ot = LoginData::Other(3);
    println!("{:?}", ot);
}
