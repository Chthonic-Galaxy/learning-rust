use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn default_author(&self) -> String {
        format!("{} is cool author DEFInitely", self.summarize_author())
    }

    fn summarize_headline(&self) -> String;
}

struct NewArticle {
    headline: String,
    author: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize_headline(&self) -> String {
        format!("{}", self.headline)
    }
}

// fn note<T: Summary>(item: &T) { ... }
fn note(item: &impl Summary) {
    println!("Note about: {}", item.summarize());
}

// fn some_func<T: Display + Clone, U: Display + Clone>(i1: T, i2: U) -> String {
//     todo!()
// }
fn some_func<T, U>(i1: T, i2: U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{
    todo!()
}

// Compile
// fn returns_summarizable() -> impl Summary {
//     NewArticle { ... }
// }
// Don't compile
// fn switch(flag: bool) -> impl Summary {
//     if flag {
//         NewsArticle { ... }
//     } else {
//         Tweet { ... }
//     }
// }

struct Pair<T> {
    value: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        todo!()
    }
}

// Don't compile - Orphan Rule!!! ToString trait isn't mine, and Summary it is only used like boundary
// impl<T: Summary> ToString for T {
//     fn to_string(&self) -> String {
//         format!("{}{}", self.summarize_author(), self.summarize_headline())
//     }
// }

fn main() {
    let article = NewArticle {
        headline: "The wild Forest".to_string(),
        author: "Wincent Fagini".to_string(),
    };
    println!("{}", article.summarize());
    println!("{}", article.default_author());

    note(&article);

    println!("{}", article.to_string())
}
