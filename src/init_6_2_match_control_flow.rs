fn main() {
    // CODE 1 - Matching on an enum

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // using curly braces for more complex match
    fn value_in_cents_v2(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                // curly braces are used to run multiple lines of code in a match arm
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // matching on an enum with data

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_v3(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    // CODE 2 - Matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:#?}", six);
    println!("none: {:#?}", none);

    // CODE 3 - Catch-All Patterns and the _ Placeholder

    let dice_roll = 9;
    let temp = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // other is a catch-all pattern
    };

    fn add_fancy_hat() {
        println!("Adding fancy hat");
    }
    fn remove_fancy_hat() {
        println!("Removing fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("Moving player {num_spaces} spaces");
    }
    fn reroll() {
        println!("Rerolling");
    }

    // catch all with _

    let dice_roll = 9;

    _ = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    };

    // catch all with _ and ()

    let dice_roll = 9;
    _ = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
