fn main() {
    // 4.3 The Slice Type

    // The string index problem

    // Slices let you reference a contiguous sequence of elements in a collection rather than the
    // whole collection. A slice is a kind of reference, so it does not have ownership.
    //
    // Here’s a small programming problem: write a function that takes a string of words separated
    // by spaces and returns the first word it finds in that string. If the function doesn’t find
    // a space in the string, the whole string must be one word, so the entire string should be returned.
    //
    // Let’s work through how we’d write the signature of this function without using slices, to understand the problem that slices will solve:

    // CODE 1 - String Indexing Problem
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // CODE 2 - String Slices

    // A string slice is a reference to part of a String, and it looks like this:

    println!("\nCODE 2:");
    let s = String::from("hello world");
    let len = s.len();

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    // print slice
    println!("hello = {}, slice = {}", hello, slice);

    // CODE 3 - Solution to String Indexing Problem
    println!("\nCODE 3:");

    fn first_word_using_slices(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    let mut s = String::from("hello world");

    let word = first_word_using_slices(&s);

    // UNCOMMENT 2 LINES to see Error!
    // s.clear(); // error!
    // println!("the first word is: {word}");

    // CODE 4 - Other Slices
    println!("\nCODE 4:");
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
