fn main() {
    use std::collections::HashMap;

    let mut armory = HashMap::new();

    armory.insert(String::from("Mosin"), 800.0);
    armory.insert(String::from("Glock-19"), 120.0);

    let some_name = String::from("Mosin");
    let velocity = armory.get(&some_name).copied().unwrap_or(0.0);

    println!("{velocity}");

    for (key, value) in &armory {
        println!("{key}: {value}");
    }

    let names = armory.keys();
    println!("{names:?}");

    let field_name = String::from("m444");
    let field_value = String::from("ZXC");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{map:?}");

    // println!("{field_name}")

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in "hello!".chars().enumerate() {
        println!("{h:?}");
        h.entry(c).or_insert(Vec::new()).push(i);
    }

    let mut sum = 0;

    for i in h.get(&'l').unwrap() {
        println!("{i}");
        sum += *i;
    }

    println!("{}", sum);
}
