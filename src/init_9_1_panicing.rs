fn main() {
    panic!("crash and burn");

    // this also panics
    let v = vec![1, 2, 3];

    v[99];
}