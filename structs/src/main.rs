struct User {
    username: String,
    email: String,
    sign_n_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("ida@gmail.com"),
        username: String::from("ida"),
        active: true,
        sign_n_count: 1

    };

    let name = user1.username;
    user1.username = String::from("Ida Delphine");

    let user2 = build_user(String::from("idadel@gmal.com"), String::from("idadel"));

    let user3 = User {
        email: String::from("idadelphine@gmail.com"),
        username: String::from("idadelphine"),
        ..user2
    };
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
} 