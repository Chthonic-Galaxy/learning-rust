fn function_above(thing: &str, value: i32) {
    println!("Above, of {thing}. Value is {value}");
    another_function(33);
}

fn main() {
    another_function(23);
    let thing: &str = "house";
    let value: i32 = "22".parse().expect("LOL");
    function_above(thing, value);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}