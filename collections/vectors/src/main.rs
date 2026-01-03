use std::ops::Range;
use std::slice::Iter;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(7);
    println!("{:?}", v);
    v.extend_from_slice(&[6, 6, 6]);
    v.extend_from_slice(&[1, 0, 8]);
    println!("{:?}", v);
    let v_m = vec![1, 2, 3];
    let mut vvi: Vec<Vec<i32>> = Vec::new();
    vvi.push(vec![1, 2, 3]);
    println!("{:?}", vvi);

    let first = v[0];
    let vs = vec![String::from("Apostal"), String::from("Yuri")];
    let firsts = &vs[0];
    println!("{first}");
    println!("{firsts}");
    println!("{:?}", v);
    println!("{:?}", vs);

    let second_b_n_b_i: Option<&String> = vs.get(1);
    let test_bad: Option<&String> = vs.get(3);
    println!("{}", second_b_n_b_i.unwrap()); // Isn't safe
    println!("{}", second_b_n_b_i.unwrap_or(&String::from("Yuji"))); // Better

    // Uncomment for testing why I comment them so (dear CloudFlare...)
    // println!("{}", test_bad.unwrap_or(&String::from("Yuji"))); // Better
    // println!("{}", test_bad.unwrap()); // Isn't safe

    // Good too
    match second_b_n_b_i {
        Some(val) => println!("{}", val),
        None => {}
    }

    // Good too
    // match test_bad {
    //     Some(val) => println!("{}", val),
    //     None => println!("Yuji"),
    // }

    for i in &v {
        println!("{i}");
    }

    let mut v_dopelg = v.clone();

    for i in &mut v_dopelg {
        *i += 50;
    }

    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();

    let mut v: Vec<i32> = vec![1, 2];
    let mut iter_r: Range<usize> = 0..v.len();
    let i1: usize = iter_r.next().unwrap();
    let n1: &i32 = &v[i1];

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
