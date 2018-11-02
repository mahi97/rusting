struct Rect {
    w : f32,
    h : f32,
}

impl Rect {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}


fn main() {
    let r = Rect { w:30.0, h: 40.0};
    println!("Rect Area : {}", r.area());
}
