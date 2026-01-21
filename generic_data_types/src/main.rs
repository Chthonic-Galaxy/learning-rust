struct OwnType {}

struct Container<T1, T2> {
    item1: T1,
    item2: T2,
}

impl<T1, T2> Container<T1, T2> {
    fn show_item1(&self) -> &T1 {
        &self.item1
    }
}

impl Container<i32, i32> {
    fn sum_i32(&self) -> i32 {
        self.item1 + self.item2
    }
}

impl Container<OwnType, OwnType> {
    fn say_it(&self) {
        println!("Good boy")
    }
}

fn main() {
    let container1 = Container {
        item1: "Io",
        item2: 32,
    };
    let container2 = Container {
        item1: 32,
        item2: 64,
    };

    println!("{}", container1.show_item1());
    println!("{}", container2.sum_i32());

    let pleasuretainer = Container {
        item1: OwnType {},
        item2: OwnType {},
    };

    pleasuretainer.say_it();
}
