fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("{guess}");
    // let unsigned_integer: u32 = 32200;
    // println!("{unsigned_integer}");
    // let signed_integer: i32 = 100;
    // println!("{signed_integer}");

    // //differnet case with negative sign numbers
    // let signed_integer: i32 = -100; //support sign
    // println!("{signed_integer}");
    
    // //let unsigned_integer: u32 = -32200; //doesn't support sign
    // //println!("{unsigned_integer}");

    // //overflow test
    // //check loop
    // let mut n: u8 = 0;
    // loop {
    //     n += 1;
    //     println!("{n}");
    //     if n == 255 {
    //         break;
    //     }
    // }
    // let float_num = 0.4;
    // println!("{float_num}");

    // let x = 2.0;
    // let y: f32 = 3.0;

    // // Numeric Operations
    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // let renainder = 43 % 5;


    // let t = true;
    // let f: bool = false; // with exolicit type annotation

    // let c = 'z';
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';

    // let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 = 0;
    // tup.1 += 5.0;
    // println!("{}", tup.0);

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", a[0]);

    // let arr_same_nums = [3; 5];
    // println!("{}", arr_same_nums[4]);

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0])
}