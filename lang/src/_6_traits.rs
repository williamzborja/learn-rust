use std::fmt::{Debug, Display};

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

trait SummaryWithManyGenerics<T, U>
where
    T: Display + Clone,
    U: Clone + Debug,
{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
#[allow(dead_code)]
fn notify(item: &impl Summary) {
    println!("{}", item.summarize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_in_parameter() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        notify(&tweet);
    }
}
