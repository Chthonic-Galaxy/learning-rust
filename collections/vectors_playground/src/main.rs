fn main() {
    let mut v: Vec<String> = Vec::with_capacity(10);
    println!("cap: {}, len: {}", v.capacity(), v.len());
    for i in ["Apos", "fix", "Yare", "Du", "Land"] {
        v.push(String::from(i));
    }
    println!("cap: {}, len: {}", v.capacity(), v.len());
    for i in ["Saviel", "Light", "New World", "rid", "Secret Island", "Project C......d"] {
        v.push(String::from(i));
    }
    println!("cap: {}, len: {}", v.capacity(), v.len());
    println!("vec: {:?}", v);
    let keep = 0 .. v.len();
    let mut iter_keeper = keep.into_iter();
    v.retain(|_| iter_keeper.next().unwrap() % 2 == 1);
    println!("cap: {}, len: {}", v.capacity(), v.len());
    println!("vec: {:?}", v);
}
