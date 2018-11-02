fn main() {
    println!("Hello, world!");
    another_function();
    mahi(5, 6);

    let y = {
        let x = 3;
        x + 1
    };
}

fn another_function() {
    println!("Another Function");
}

fn mahi(x: i32, y: i32) {
    println!("X, Y is {}, {}", x, y);
}

fn five() -> i32 {
    5
}

fn pluse_one(x :i32) -> i32 {
    x + 1
}
