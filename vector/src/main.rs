fn main() {
    
    let v = vec![1,2,3,4];
    let mut v2 : Vec<isize> = Vec::new();
    let mut m_v : Vec<isize> = Vec::new();

    let mut v_index : usize = 3;
    match v.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); },
        None => { println!("Unreachable element at index: {}", v_index); }
    }
    v_index = 5;
    match v.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); },
        None => { println!("Unreachable element at index: {}", v_index); }
    }

    let does_not_exist = v.get(100);

    v2.push(1);
    m_v.push(5);

    println!("Hello, world!");
}
