fn main() {
    // CODE 1 - Creating a New String

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // The method also works on a literal directly:
    let s = String::from("initial contents");

    // Remember that strings are UTF-8 encoded, so we can include any
    // properly encoded data in them, as shown in Listing 8-14.

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // CODE 2 - Updating a String

    let mut s = String::from("foo");

    s.push_str(" bar!");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // immutable borrow occurs here
    s1.push_str(s2);

    // immutable borrow occurs here
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    // CODE 3 - Concatenation with the + Operator or the format! Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("s1: {s1:?}") // this will throw error

    /*
    In the standard library, you’ll see add defined using generics and associated types
    Here, we’ve substituted in concrete types, which is what happens when we call this method with String values
    We’ll discuss generics in Chapter 10
    This signature gives us the clues we need in order to understand the tricky bits of the + operator.

    First, s2 has an &, meaning that we’re adding a reference of the second string to the first string
    This is because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together
    But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add
    So why does Listing 8-18 compile?

    The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str
    When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    We’ll discuss deref coercion in more depth in Chapter 15
    Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.

    Second, we can see in the signature that add takes ownership of self because self does not have an &
    This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that
    So, although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result
    In other words, it looks like it’s making a lot of copies, but it isn’t; the implementation is more efficient than copying.

     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // s1 cannot be used now but s2 and s3 are free for mutation


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // this is like println! but it returns a string instead of printing it 
    // and doesn't take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}");

    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");
    println!("s is {s}");

    // CODE 4 - Indexing into Strings

    let s1 = String::from("hi");
    
    // the type `str` cannot be indexed by `{integer}`
    // the trait `SliceIndex<str>` is not implemented for `{integer}`, which is required by `String: Index<_>`
    // you can use `.chars().nth()` or `.bytes().nth()`
    // for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
    // the trait `SliceIndex<[_]>` is implemented for `usize`
    // for that trait implementation, expected `[_]`, found `str`
    // required for `String` to implement `Index<{integer}>`
    
    // uncommenting this will throw an error at compile time
    // let h = s1[0];


    // CODE 5 - Bytes and Scalar Values and Grapheme Clusters! Oh My!

    // u8 values that looks like this:
    // ['न', 'म', 'स', '्', 'त', 'े']

    // grapheme clusters that looks like this:
    // ["न", "म", "स्", "ते"]
    
    // what should index 4 return?
    // should it be '्' or 'ते'?

    // CODE 6 - Slicing Strings

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // this will throw an error at runtime
    // byte index 5 is not a char boundary; it is inside 'р' (bytes 4..6) of `Здравствуйте`
    // let s = &hello[0..=4];
    
    println!("s is {s}");


    // BEST PRACTICE - Specify the char or bytes

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // CODE 7 - Methods for Iterating Over Strings

    let data = "initial contents";
    
}
