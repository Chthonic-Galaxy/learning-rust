fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");
    let unsigned_integer: u32 = 32200;
    println!("{unsigned_integer}");
    let signed_integer: i32 = 100;
    println!("{signed_integer}");

    //differnet case with negative sign numbers
    let signed_integer: i32 = -100; //support sign
    println!("{signed_integer}");
    
    //let unsigned_integer: u32 = -32200; //doesn't support sign
    //println!("{unsigned_integer}");

    //overflow test
    //checlom
    let mut n: u8 = 0;
    loop {
        n += 1;
        println!("{n}");
    }
}