fn main() {
    // 5.3 Method Syntax
    // CODE 1 - Method Syntax

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> u32 {
            self.width
        }
        fn height(&self) -> u32 {
            self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // CODE 2 - Methods with More Parameters

    #[derive(Debug)]
    struct Rectangle2 {
        width: u32,
        height: u32,
    }

    impl Rectangle2 {
        fn can_hold(&self, other: &Rectangle2) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle2 {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle2 {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // CODE 3 - Associated Functions

    #[derive(Debug)]
    struct Rectangle3 {
        width: u32,
        height: u32,
    }

    impl Rectangle3 {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle3::square(3);

    // let sq2 = Rectangle3.square(3); // this will throw an error
    
    println!("sq is {sq:?}");

    // Multiple impl blocks

    #[derive(Debug)]
    struct Rectangle4 {
        width: u32,
        height: u32,
    }

    impl Rectangle4 {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    impl Rectangle4 {
        fn can_hold(&self, other: &Rectangle4) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle4::square(3);

    let area = rect1.area(); // this is possible because the area method doesnt return Self

    println!("area is {area}");

}
