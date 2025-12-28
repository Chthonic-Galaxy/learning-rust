// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     println!("FW is {word}");
//     s.clear();

// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn main() {
    let x = Box::new(0);
    let y = Box::new(&x);

    let b = ***y;
    assert_eq!(b, 0);
    println!("{}", b);


  println!(
    "&String={} &str={}",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );


}