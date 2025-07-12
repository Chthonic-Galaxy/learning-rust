fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first);
    println!("{full}, originally {first_clone}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}