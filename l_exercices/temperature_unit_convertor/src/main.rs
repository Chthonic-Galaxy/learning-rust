fn unit_convertor(unit: &str, value: f32) -> f32 {
    if unit == "farh" {
        (value - 32.0) * 5.0/9.0
    } else if unit == "cel" {
        value * 1.8 + 32.0
    } else {
        value
    }
}

fn main() {
    let val = unit_convertor("cel", 19.44445);
    println!("The value is {val}");
}
