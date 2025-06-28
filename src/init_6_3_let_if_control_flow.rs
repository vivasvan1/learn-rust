fn main() {
    // CODE 1 - Concise Control Flow with if let and let else

    // how to do it with match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // how to do it with if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // this is same as match with the first arm and the catch all arm is automatically assume to be ()

    // you can also use if let with "else"

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let mut count: i32 = 0;
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("count: {count}");

    // CODE 2 - Staying on the “Happy Path” with let...else

    impl UsState {
        fn existed_in(&self, year: i32) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old, for America!"))
            } else {
                Some(format!("{state:?} is relatively new."))
            }
        } else {
            None
        }
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let state_description = describe_state_quarter(coin);
    println!("state_description: {state_description:?}");

    // That gets the job done, but it has pushed the work into
    // the body of the if let statement, and if the work to be done is more complicated,
    // it might be hard to follow exactly how the top-level branches relate.
    // We could also take advantage of the fact that expressions
    // produce a value either to produce the state from the if let or to return early,
    // as in Listing 6-8. (You could do similar with a match, too.)

    fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
        let state: UsState = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    let coin2 = Coin::Quarter(UsState::Alaska);
    let state_description2 = describe_state_quarter_v2(coin2);
    println!("state_description2: {state_description2:?}");

    // This is a bit annoying to follow in its own way,
    // though! One branch of the if let produces a value,
    // and the other one returns from the function entirely.

    // let...else

    fn describe_state_quarter_v3(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    let coin3 = Coin::Quarter(UsState::Alaska);
    let state_description3 = describe_state_quarter_v3(coin3);
    println!("state_description3: {state_description3:?}");

    // CODE 3 - while let Conditional Loops

    let mut stack = Vec::new();
    stack.push(1);
}
