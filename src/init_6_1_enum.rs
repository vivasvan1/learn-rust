fn main() {
    // CODE 1 - Defining an Enum

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:#?}", four);
    println!("six: {:#?}", six);

    // you might be tempted to tackle this problem (storing ip address) with structs as shown in Listing 6-1.

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // However, this approach is a bit awkward: two separate structs with the same name and fields.
    // An enum would be a better way to model this, as we can list all possible types of IP addresses

    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let _home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddr2::V6(String::from("::1"));

    // Even better enum

    #[derive(Debug)]
    enum IpAddr3 {
        // 127.0.0.1
        V4(u8, u8, u8, u8),
        // 2001:db8:3333:4444:5555:6666:7777:8888
        V6(String),
    }

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    // 2001:db8:3333:4444:5555:6666:7777:8888
    let loopback3 = IpAddr3::V6(String::from("::1"));

    println!("home3: {:#?}", home3);
    println!("loopback3: {:#?}", loopback3);

    // more complex example

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Message::call");
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Write(String::from("hello"));
    println!("m: {:#?}", m);

    // The Option Enum and Its Advantages Over Null Values

    let some_number = Some(5);
    let some_char = Some('e');

    println!("some_number: {:#?}", some_number);
    println!("some_char: {:#?}", some_char);

    let mut absent_number: Option<u32> = None;

    println!("absent_number: {:#?}", absent_number);

    absent_number = Some(5);

    println!("absent_number: {:#?}", absent_number);

    // this code wont compile as option can be none ans compiler knows it
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    
    // let sum = x + y;

    // convert option<t> -> t safely
    match y {
        Some(val) => {
            let sum = x + val;
            println!("sum: {}", sum);
        }
        None => {
            println!("Cannot calculate sum: y is None.");
        }
    }





}
