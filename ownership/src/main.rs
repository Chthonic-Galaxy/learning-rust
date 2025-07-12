fn main() {
    let n = 5;
    let y = minus_one(n);

    println!("The value of y={y}, and of n={n}");
}

fn minus_one(x: i32) -> i32{
    x - 1
}