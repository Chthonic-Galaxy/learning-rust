fn gen_fibonacci(n: u16) -> u32 {
    let (mut p, mut r): (u32, u32) = (0, 1);
    for _ in 0..n {
        (p, r) = (r, p + r);
    }
    r
}

fn main() {
    let n = 34;
    println!("The {n} Fibonacci is {}", gen_fibonacci(n));
}
