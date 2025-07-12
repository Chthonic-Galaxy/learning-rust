fn main() {
    let a= Box::new([1; 1000000]);
    let b = Box::new([1; 1000000]);

    for i in a.iter() {
        println!("{i}")
    }
}