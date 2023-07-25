struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = build_user(
        String::from("connor@connomation.ca"),
        String::from("Connor"),
    );

    // NOTE: doing this would invalidate user, since the username String gets used up in the copying
    // let mut user2 = User {
    //     email: String::from("connor@connomation.ca"),
    //     ..user
    // };
    // user2.username = String::from("Test");

    // NOTE: this will NOT invalidate user, since active and sign_in_count can be copied without having to move them, and we didn't use any of user's String properties.
    let user2 = User {
        email: String::from("connor@flightpath.fm"),
        username: String::from("Work"),
        ..user
    };

    println!(
        "User is User {} {} {} {}",
        user.username, user.email, user.active, user.sign_in_count
    );
    println!("User2 is {}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
