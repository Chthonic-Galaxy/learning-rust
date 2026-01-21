use std::fs::{self, File};
use std::io::{self, Read};
// use std::io::ErrorKind;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("wow.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e ),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("wow.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("wow.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

fn last_char_of_alph_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("wow.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    // let g_file_result = File::open("wow.txt");

    // let g_file = match g_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("wow.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Problem createing the file {error:?}"),
    //         },
    //         _ => panic!("Problem opening the file: {error:?}"),
    //     },
    // };

    // let g_file = g_file_result.unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("wow.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // let g_file_result = File::open("wow.txt").unwrap();
    // let g_file_result = File::open("wow.txt")
    //     .expect("wow.txt should bi included in this project");

    let g_file = File::open("wow.txt")?;

    Ok(())
}
