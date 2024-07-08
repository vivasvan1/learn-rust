pub fn main() {
    // CODE 1 - Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    /*          ^--- This is the data type */

    /*
    Scalar Types
    A scalar type represents a single value. Rust has four primary
    scalar types: integers, floating-point numbers, Booleans, and
    characters. You may recognize these from other programming languages.
    Let’s jump into how they work in Rust.
     */

    // CODE 2 - Integers
    let x: i8 = -128;
    let x: i8 = 127;

    let x: i16 = -32_768;
    let x: i16 = 32_767;

    let x: i32 = -2_147_483_648;
    let x: i32 = 2_147_483_647;

    let x: i64 = -9_223_372_036_854_775_808;
    let x: i64 = 9_223_372_036_854_775_807;

    let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let x: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

    // isize is the same as i64 on 64-bit platforms and i32 on 32-bit platforms
    let x: isize = -9_223_372_036_854_775_808;
    let x: isize = 9_223_372_036_854_775_807;

    // CODE 3 - Literals
    let decimal = 42;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // CODE 4 - Floating-point numbers
    let f = 3.14;
    let f: f32 = 3.14;

    // CODE 5 - Numerical operations

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Result is -1

    let remainder = 43 % 5;

    // CODE 6 - Booleans
    let t = true;
    let f: bool = false;

    // CODE 7 - Characters
    let c = 'a';
    let c: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // CODE 8 - Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    // CODE 9 - Arrays

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3, 5];

    let first = a[0];
    let second = a[1];

}
