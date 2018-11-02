fn main() {

    println!("Hello, world!");
    // IF STATEMENT
    let mut x = 3;
    if x != 0 { // Can not use x as a condition
        println!("X is not zero");
    }
    let condition = true;
    let y = if condition {
        5
    } else {
        6
    };

    println!("Y is {}", y);
    // WHILE LOOP
    while x > 0 {
        x = x - 1;
        println!("X is {}", x);
    }

    //FOR LOOP
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("A elemnt : {}", i);
    }

    for i in (1..4).rev() {
        println!("{}!", i);
    }

    // LOOOOOP
    //loop {
    //    println!("Again!");
    //}
}
