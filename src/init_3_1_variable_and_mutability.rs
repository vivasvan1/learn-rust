pub fn main() {
    // CODE 1
    // let x = 5;
    // println!("x has the value {}", x);

    // x = 6;
    // println!("x has the value {}", x);

    // OUTPUT CODE 1
    /*
       error[E0384]: cannot assign twice to immutable variable `x`
       --> init_3_1_variable_and_mutability.rs:6:5
       |
       3 |     let x = 5;
       |         -
       |         |
       |         first assignment to `x`
       |         help: consider making this binding mutable: `mut x`
       ...
       6 |     x = 6;
       |     ^^^^^ cannot assign twice to immutable variable

       error: aborting due to 1 previous error

       For more information about this error, try `rustc --explain E0384`.
    */

    // CODE 2
    let mut x = 5;
    println!("x has the value {}", x);

    x = 6;
    println!("x has the value {}", x);

    // OUTPUT CODE 2
    /*
       x has the value 5
       x has the value 6
    */

    // CONSTANTS

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // OUTPUT CODE 3
    /*
       Three hours in seconds: 10800
    */

    // CODE 4 - Shadowing

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in inner scope is : {x}");
    }

    println!("the value of x in outer scope is : {x}");

    // OUTPUT CODE 4
    /*
       the value of x in inner scope is : 6
       the value of x in outer scope is : 12
    */

    //
}
