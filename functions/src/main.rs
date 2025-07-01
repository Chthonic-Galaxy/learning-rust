fn function_above() {
    println!("Above^");
}

fn main() {
    another_function();
    function_above();
}

fn another_function() {
    println!("Another function.");
}