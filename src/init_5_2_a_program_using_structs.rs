fn main() {
    // 5.2 A Program Using Structs
    // CODE 1 - Defining a Struct
    println!("\nCODE 1:\n");

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    // CODE 2 - Refactoring with Tuples

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    // CODE 3 - Refactoring with Structs: Adding More Meaning

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    // CODE 4 - Adding Useful Functionality with Derived Traits

    struct Rectangle2 {
        width: u32,
        height: u32,
    }

    let rect2 = Rectangle2 {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect2); // error as Debug trait is not implemented for Rectangle2

    #[derive(Debug)]
    struct Rectangle3 {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle3 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // works as Debug trait is implemented for Rectangle3

    // using dbg! macro

    let scale = 2;
    let rect1 = Rectangle3 {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
