struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
    };
    // User 2 is created using some value of user 1
   let user2 = User {
        active: user1.active,
        username: String::from("username2"),
        email: String::from("email2@email.com"),
        sign_in_count: user1.sign_in_count,
    };
    // User 3 is also created using some value of user 1 but not specifying everything 
    let user3 = User {
        username: String::from("username3"),
        email: String::from("email3@email.com"),
        ..user1
    };

    println!("{0} has this email: {1}",user1.username, user1.email);
    println!("{0} has this email: {1}",user2.username, user2.email);
    println!("{0} has this email: {1}",user3.username, user3.email);

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}