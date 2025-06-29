fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<
    T: std::cmp::PartialOrd
>(
    list: &[T],
) -> &T {
    let mut largest = &list[0];

    for item in list {
        // This wont compile for all types if you remove the PartialOrd trait above.
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // CODE 1 : Generic Data Types
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // using generic data types
    println!("The largest number is {}", largest(&number_list));
    println!("The largest char is {}", largest(&char_list));

    // CODE 2 : In Struct Definitions

    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // This wont work because x and y are of different types
    // let wont_work = Point { x: 5, y: 4.0 };

    // This will work because x and y are of different types
    struct PointV2<T, U> {
        x: T,
        y: U,
    }

    let both_integer = PointV2 { x: 5, y: 10 };
    let both_float = PointV2 { x: 1.0, y: 4.0 };
    let integer_and_float = PointV2 { x: 5, y: 4.0 };

    // CODE 3 : In Enum Definitions

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // CODE 4 : In Method Definitions

    struct PointV3<T> {
        x: T,
        y: T,
    }

    impl<T> PointV3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl PointV3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = PointV3 { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p2 = PointV3 { x: 1.0, y: 2.0 };
    println!("Distance from origin = {}", p2.distance_from_origin());

    // CODE 5 : mixup

    struct PointV4<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> PointV4<X1, Y1> {
        fn mixup<X2, Y2>(self, other: PointV4<X2, Y2>) -> PointV4<X1, Y2> {
            PointV4 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = PointV4 { x: 5, y: 10.4 };
    let p2 = PointV4 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
