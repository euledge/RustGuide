struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@mail"),
        username: String::from("somebody_name"),
        active: false,
        sign_in_count: 0,
    };
    println!("{}", user.username);
    let mut mutable_user = User {
        email: String::from("someone@mail"),
        username: String::from("somebody_name"),
        active: false,
        sign_in_count: 0,
    };
    println!("{}", mutable_user.username);
    mutable_user.username = String::from("anybody_name");
    println!("{}", mutable_user.username);

    let build_user = build_user(String::from("someone"), String::from("anybody@mail"));
    println!("build_user is: {}", build_user.username);

    {
        let email = String::from("someone@mail");
        let user = User {
            email,
            username: String::from("somebody_name"),
            active: false,
            sign_in_count: 0,
        };
        println!("{}", user.email);
    }

    {
        let user1 = User {
            email: String::from("someone@mail"),
            username: String::from("somebody_name"),
            active: false,
            sign_in_count: 0,
        };
        println!("user1 {}", user1.email);
        let user2 = User { ..user1 };
        println!("user2 {}", user2.email);
        println!("user1 {}", user1.email);
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        email: email,
        username: username,
        active: false,
        sign_in_count: 1,
    }
}
