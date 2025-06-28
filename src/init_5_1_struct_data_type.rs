fn main() {
    // 5.1 Struct Data Type

    // CODE 1 - Defining a Struct
    println!("\nCODE 1:\n");

    // Defining a struct
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Instantiating a struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // cannot mutate un-mutable struct
    // user1.email = String::from("anotheremail@example.com"); //error

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("This is struct printed: \n{:#?}", user1);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(
        String::from("anotheremail@example.com"),
        String::from("anotherusername123"),
    );

    // CODE 2 - Using the Field Init Shorthand

    println!("\nCODE 2:");

    fn build_user_2(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // CODE 3 - Creating Instances from Other Instances with Struct Update Syntax

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Above code is same as
    let user2 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // CODE 4 - Using Tuple Structs Without Named Fields to Create Different Types

    println!("\nCODE 4:");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // CODE 5 - Unit-Like Structs Without Any Fields

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    

}
