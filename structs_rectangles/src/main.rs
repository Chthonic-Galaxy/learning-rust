#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn increse_evth_by_two(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, some_rectangle: &Rectangle) -> bool {
        if self.width > some_rectangle.width && self.height > some_rectangle.height {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    if rect1.width() {
        rect1.increse_evth_by_two();
        println!(
            "The area of the rectangle is {}, square pixels.",
            rect1.area()
        );
    }
    dbg!(&rect1);
    let rect2 = Rectangle {
        width: 50,
        height: 90,
    };
    println!("{}",rect2.can_hold(&rect1));
}