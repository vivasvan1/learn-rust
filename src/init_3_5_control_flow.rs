fn main() {
    // CODE 1 - IF EXPRESSION
    println!("\nCODE 1:");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // CODE 2 - IF CONDITION MUST BE A BOOLEAN EXPRESSION
    println!("\nCODE 2:");

    // THROWS AN ERROR
    // if number {
    //     println!("number was three");
    // }

    // WORKS
    if number != 0 {
        println!("number was something other than zero");
    }

    // CODE 3 - IF EXPRESSION WITH MULTIPLE CLAUSES
    println!("\nCODE 3:");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // CODE 4 - IF IN A LET STATEMENT
    println!("\nCODE 4:");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // CODE 5 - BOTH ARMS OF AN IF STATEMENT MUST BE OF THE SAME TYPE

    // THROWS AN ERROR
    // let number = if condition { 5 } else { "six" };

    /* CODE 6 - LOOPS */

    /* WARNING - INFINITE LOOP */
    // loop {
    //     println!("again!");
    // }

    // CODE 7 - LOOPS WITH A CONDITION
    println!("\nCODE 7:");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // CODE 8 - LOOP LABELS
    println!("\nCODE 8:");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // CODE 9 - WHILE LOOP
    println!("\nCODE 9:");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // CODE 10 - LOOPING THROUGH A COLLECTION WITH FOR
    let a = [10, 20, 30, 40, 50];

    // LOOPING THROUGH A COLLECTION USING WHILE
    let mut index = 0;
    while index < 5 {
        println!("using while: the value is: {}", a[index]);
        index += 1;
    }

    // MUCH MORE READABLE WITH FOR
    for element in a {
        println!("using for: the value is: {element}");
    }

    // CODE 11 - LOOPING THROUGH A COLLECTION WITH A RANGE
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
