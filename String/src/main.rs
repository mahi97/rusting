fn main() {
    let mut s = String::new();
    let data = "Mahi is The Best";
    s = data.to_string();
    println!("{}", s);
    let s2 = String::from("Hey There");
    let s3 = "from string Literal".to_string();
    println!("Hello, world!, {}, {}", s2, s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    
    let t1 = "tik".to_string();
    let t2 = "tok".to_string();
    let t3 = "toe".to_string();

    let s4 = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", s4);
   
    for c in s4.chars() {
        println!("{}", c);
    }
    println!("{}", &t1[0..1]);

}
