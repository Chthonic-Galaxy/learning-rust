fn main() {
    let number = 9;

    if number % 4 == 0 {
        println!("Number is devisible by 4")
    } else if number % 3 == 0 {
        println!("Number is devisible by 3")
    } else {
        println!("Number is not divisible by either 4 or 3")
    }
}