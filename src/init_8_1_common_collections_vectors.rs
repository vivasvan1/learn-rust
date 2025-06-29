fn main() {
    // CODE 1 - Creating a New Vector

    let v: Vec<i32> = Vec::new();

    // automatically infer the type of the vector from the values
    let v = vec![1, 2, 3];

    // updating a vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // this will panic at runtime
    // println!("does_not_exist: {does_not_exist:?}");
    let does_not_exist2 = v.get(100);
    println!("does_not_exist2: {does_not_exist2:?}");

    // This code will throw an error at compile time
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // this is mutable borrow, so we can't borrow it again for v.push below
    let first = &v[0];

    // this will throw an error at compile time as it is trying to borrow v as mutable but it is already borrowed as immutable
    v.push(6);

    // uncommenting this will throw an error at compile time as it is trying to borrow v as immutable but it is already borrowed as mutable
    // println!("The first element is: {first}");

    // CODE 2 - Iterating Over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element
    // in a mutable vector in order to make changes to all the elements.
    // The for loop in Listing 8-8 will add 50 to each element.

    let mut v: Vec<i32> = vec![100, 32, 57];

    println!("The values of v are: {v:?}");

    // mutable borrow occurs here
    for i in &mut v {
        *i += 50;
    }

    println!("The values of v are: {v:?}");

    // CODE 3 - Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // CODE 4 - Dropping a Vector Drops Its Elements

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
