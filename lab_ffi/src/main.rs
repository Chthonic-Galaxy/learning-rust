unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    let abs = unsafe { abs(-5 as i32) };
    println!("{abs}")
}
