#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin,
}

impl PermissionLevel {
    fn is_admin(&self) -> bool {
        let ret = if let PermissionLevel::Admin = self {
            true
        } else {
            false
        };

        ret
    }

    fn is_admin2(&self) -> bool {
        if let PermissionLevel::Admin = self {
            true
        } else {
            false
        }
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("1{}", user1.is_admin());
    println!("2{}", user1.is_admin2());

    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.is_admin());

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.is_admin());
}
