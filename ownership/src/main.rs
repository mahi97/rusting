fn main() {
    println!("Hello, world!");
    let s1 = String::from("Mahi");
    let s2 = s1.clone();
    println!("S2: {}", s2);
    println!("S1: {}", s1);
    let l = check(&s2);
    println!("LEN {}", l);

    let mut a = 5;
    let r = &mut a;
    let r2 = &mut a;
}

fn check(s :&String) -> usize {
    s.len()
}
