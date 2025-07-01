fn function_before(thing: &str, value: i32) {
    println!("Above, of {thing}. Value is {value}");
    another_func_after(33);
}

fn func_implicit(value: &str) -> i32 {
    value.parse::<i32>().expect("Impresive, LOL")
}

fn func_explicit(name: &str, surname: &str) -> String {
    let fullname = format!("{name} {surname}").to_owned();
    return fullname
}

fn main() {
    another_func_after(23);
    let thing: &str = "house";
    let value: i32 = func_implicit("22");
    function_before(thing, value);
    println!("{}", func_explicit("Tom", "Voznik"));

    let y = {
        let h = 6;
        h + 1
    };
    println!("{y}")
}

fn another_func_after(x: i32) {
    println!("The value of x is: {x}");
}