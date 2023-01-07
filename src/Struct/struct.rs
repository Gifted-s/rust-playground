struct User {
    name: String,
    email: String,
    active: bool,
    signincount: u64,
}

fn main() {
    let mut user1 = User {
        name: String::from("Adewumu Sunkanmi"),
        email: String::from("sunkaniadewumi1@gmail"),
        active: false,
        signincount: 30,
    };

    let name = user1.name;
    user1.name = String::from("Adewumi Sunkanmi");
    let user2 = builduser(
        String::from("Adewumi Sunkanmi"),
        String::from("sunkanmiadewumi1@gmail.com"),
    );

    let mut user3 = User {
        name: String::from("Adewumu Sunkanmi"),
        email: String::from("sunkaniadewumi1@gmail"),
        ..user2
    };

    // Tuple struct 
    struct Color(u64, u64, u64);
    struct Point (i64, i64, i64);


    


}

fn builduser(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
        signincount: 1,
    }
}
