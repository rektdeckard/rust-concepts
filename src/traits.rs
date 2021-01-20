pub trait Summary {
    fn summarize(&self) -> String {
        // Default implementation
        String::from("(Read more)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Trait bounds
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}
