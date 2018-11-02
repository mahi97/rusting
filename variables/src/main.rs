fn main() {

    // VARIABLES
    let mut x = 5;
    println!("Hello, world!{}", x);
    x = 6;
    let x = x + 2;
    println!("NEW X: {}", x);
    let guess: u32 = "42".parse().expect("Not a number!");

    // CONST
    const MAX_INT: u32 = 1000_000;

    // DATA TYPES
    let x = 2.0;
    let mahi: i32 = 1000_000_000;
    let hex: u8 = 0xff;

    let fp: f32 = 23.3;
    let fp = 23.2;
    let f: bool = false;

    // TUPLE
    let tup: (i32, f64, bool) = (23, 23.4, true);
    let tup = (23, 0.23, false);

    let (x, y, z) = tup;
    println!("X, Y, Z is {}, {}, {}", x, y, z);

    // ARRAY
    let a = [1, 2, 3, 4, 5]; 

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


}
