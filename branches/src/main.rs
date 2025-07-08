fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Value of number: {number}");

    let number2 = if condition { 5 } else { "six" };

    println!("Value of number: {number}")
}