// struct Processor;

// impl Processor {
//     fn process_it<F>(&self, func: F)
//     where
//         // F: for<'a> Fn(&'a str) -> i32,
//         F: Fn(&str) -> i32,
//     {
//         let s = String::from("temp");
//         let res = func(&s);
//         println!("{res}")
//     }
// }

// fn main() {
//     let proc = Processor;

//     proc.process_it(|_| 6);
// }

// use std::fmt::Display;

// struct SmartCache<T> {
//     value: T,
// }

// impl<T: PartialOrd> SmartCache<T> {
//     fn new(value: T) -> Self {
//         SmartCache { value }
//     }

//     fn update(&mut self, new_val: T) {
//         if self.value < new_val {
//             self.value = new_val
//         }
//     }

//     fn get(&self) -> &T {
//         &self.value
//     }
// }

// impl<T: Display> Display for SmartCache<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }

// fn main() {
//     let mut cache = SmartCache::new("Cool edit");
//     println!("{}", cache);
//     cache.update("Super cool edit");
//     println!("{}", cache);
//     println!("{}", cache.get());
// }

#[derive(Debug)]
struct TextEditor<'a> {
    text: &'a str,
    query: Option<&'a str>,
}

impl<'a> TextEditor<'a> {
    fn search(&mut self, query: &str) -> Option<&'a str> {
        let start = self.text.find(query);
        if let Some(s) = start {
            self.query = Some(&self.text[s..s + query.len()]);
            self.query
        } else {
            self.query = None;
            None
        }
    }
}

fn find_best<T, F: Fn(&T) -> f64>(list: &[T], scorer: F) -> Option<&T> {
    let mut best: Option<&T> = None;

    if list.len() >= 1 {
        best = Some(&list[0]);
        for e in list {
            let score = scorer(e);
            if scorer(best.unwrap()) < score {
                best = Some(e);
            }
        }
        return best;
    } else {
        return None;
    }
}

fn main() {
    let text = "Some string wow";
    let query = "str";
    let mut te = TextEditor { text, query: None };
    println!("{}", te.search(query).unwrap());
    println!("{:?}", te);

    let best = find_best(&[1, 2, 3], |&x| x as f64);
    println!("{}", best.unwrap());
}
