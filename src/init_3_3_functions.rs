pub fn main() {
    // CODE 1
    println!("\nCODE 1:");
    println!("Hello, world!");
    another_function();

    // CODE 2
    println!("\nCODE 2:");
    yet_another_function(5);

    // CODE 3
    println!("\nCODE 3:");
    print_labeled_measurement(10, 'm');

    // CODE 4 - Statements and expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    let x = 5;

    // let x = (let y = 6);
    // The above statement throws an error
    // Compiling functions v0.1.0 (file:///projects/functions)
    //     error: expected expression, found `let` statement
    //      --> src/main.rs:2:14
    //       |
    //     2 |     let x = (let y = 6);
    //       |              ^^^
    //       |
    //       = note: only supported directly in conditions of `if` and `while` expressions

    // However, the following statement is valid
    let y = {
        let x = 3;
        x + 1
    };

    // Functions are also expressions

    // WORKS
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    // DOES NOT WORK
    // fn plus_one(x: i32) -> i32 {
    //     x + 1;
    // }

    



}

fn another_function() {
    println!("Another function");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {}{}", value, unit);
}
