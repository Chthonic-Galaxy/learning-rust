fn main() {
    let mut s = String::new();
    println!("s='{s}'");

    let data = "initial data";

    let s = data.to_string();

    println!("{s}");

    let s = "initial data, but other writting way".to_string();
    println!("{s}");

    let s = String::from("Initial data, but some more other way");
    println!("{s}");

    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שלום");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    let mut s = String::from("Foo ");
    s.push_str("bar added");
    println!("{s}");

    let mut s1 = String::from("Don't take ownership of this ");
    let s2 = "String";
    s1.push_str(s2);
    println!("s2 contains {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Helo ");
    let s2 = String::from("WOOOOORLD!!!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // Alternative way of doing the same action as line belowe
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let weird_word = "नमस्ते".to_string();
    //way 1
    for i in (0..weird_word.len()).skip(3).step_by(3) {
        println!("{}", &weird_word[i - 3..i]);
    }
    //way 2
    for ch in weird_word.chars() {
        println!("{ch}");
    }
    // String are not as simple as it seem
}
