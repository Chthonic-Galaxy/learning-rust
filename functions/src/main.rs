fn function_above(thing: String, value: i32) {
    println!("Above, of {thing}. Value is {value}");
    another_function(33);
}

fn main() {
    another_function(23);
    function_above();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}