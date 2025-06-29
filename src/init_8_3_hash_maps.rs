use std::collections::HashMap;

fn main() {
    // CODE 1 - Creating a New Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // CODE 2 - Accessing Values in a Hash Map

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score is {score}");

    // CODE 3 - Iterating over a Hash Map


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }



    // CODE 4 - Hash Maps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("field_name is {field_name}");
    // println!("field_value is {field_value}");

    println!("{map:?}");


    // CODE 5 - Updating a Hash Map

    let mut scores = HashMap::new();

    // Overwriting a Value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
    
    // Inserting a Key and Value if a Key Isn’t Present
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:?}");

    // Adding a Key and Value Only If a Key Isn’t Present

    // This will not update the value because the key is already present
    scores.entry(String::from("Yellow")).or_insert(500);

    // This will update the value because the key is NOT present
    scores.entry(String::from("Red")).or_insert(500);

    println!("{scores:?}");

    // CODE 6 - Updating a Value Based on the Old Value


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
