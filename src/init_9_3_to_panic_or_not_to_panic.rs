fn main() {

    let home: std::net::IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");



        pub struct Guess {
            value: i32,
        }
        
        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {value}.");
                }
        
                Guess { value }
            }
        
            pub fn value(&self) -> i32 {
                self.value
            }
        }

        let guess = Guess::new(100);

        println!("Guess value is {}", guess.value());

        let guess = Guess::new(101);

        println!("Guess value is {}", guess.value());
}