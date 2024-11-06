fn main() {
    // 4.1 Ownership

    // CODE 1 - OWNERSHIP RULES
    println!("\nCODE 1:");
    {
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // s goes out of scope here

    // CODE 2 - OWNERSHIP ON STACK AND HEAP
    println!("\nCODE 2:");
    let x = 5;
    let y = x; // x is copied to y
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
                 // println!("{s1}, world!"); // This line will cause a compile-time error

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy
    let x = 5;
    let y = x; // x is copied to y
    println!("x = {x}, y = {y}");

    // CODE 3 - Ownership and Functions
    println!("\nCODE 3:");
    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
    } // some_string goes out of scope and memory is freed

    fn makes_copy(mut some_integer: i32) {
        // some_integer comes into scope
        some_integer += 1;
        println!("{some_integer}");
    } // some_integer goes out of scope

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function

    let mut x = 5; // x comes into scope
    makes_copy(x); // x is copied into the function
    println!("{x}");

    // CODE 4 - Return Values and Scope
    println!("\nCODE 4:");
    fn gives_ownership() -> String {
        // moves its return value to the caller
        let some_string = String::from("yours");
        some_string // returned and moves out to the caller
    }

    fn takes_and_gives_back(a_string: String) -> String {
        // takes a String and returns one
        a_string // returned and moves out to the caller
    }

    let s1 = gives_ownership(); // gives_ownership moves its return value to s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved and then moved again to s3

    fn calculate_length(s: String) -> (String, usize) {
        // takes a String and returns a tuple
        let length = s.len(); // get the length of the String
        (s, length) // return the String and its length
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1 is moved and returned as s2
    println!("The length of '{s2}' is {len}.");
}
