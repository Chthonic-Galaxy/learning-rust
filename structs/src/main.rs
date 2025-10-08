struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Info(String, String, bool);

struct AlwaysEqual; 


fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("GODDAMNboy333"),
        email: String::from("damn@mail.hell"),
        sign_in_count: 1,
    };

    user1.active = false;

    println!("The OUR FIRST user is:\n{}\nemail: {}", user1.username, user1.email);

    let user2 = build_user(String::from("ww"), String::from("Cool"));
    println!("\n========\nThe user2's IFNO:\nusername: {},\nemail: {},\nsign in count: {},\nis active: {}", user2.username, user2.email, user2.sign_in_count, user2.active);

    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;

    println!("Coords: x: {x}, y: {y}, z: {z}");
    
    let info = Info(String::from("Some dude"), String::from("ZXC Ghoul 1000-7"), true);
    let Info(ref nickname, ref class, is_active) = info;
    
    println!("\n====\nNickname: {nickname}\nClass: {class}\nActive: {is_active}");
    println!("\n====\nNickname: {}\nClass: {}\nActive: {}", info.0, info.1, info.2);

    let subject = AlwaysEqual;

    // println!("{}", origin)


    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("chmonya777@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // let user34 = User {
    //     email: String::from("massacre1943@ashit.nazi"),
    //     ..user1
    // };

}
