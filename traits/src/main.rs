mod convert;
mod traits;

use traits::{notify, Article, Summary, Tweet};

fn main() {
    // TRAITS
    let tweet = Tweet {
        username: String::from("friedtm"),
        content: String::from("Only third-week 2021 kids will remember this..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = Article {
        headline: String::from("Trump discusses nuclear option with MyPillow CEO"),
        location: String::from("Washington, DC"),
        author: String::from("Someguy Personsson"),
        content: String::from("The infamous CEO of sleep brand MyPillow was seen exiting the Oval Office this afternoon, holding a document detailing plans to declare martial law in advance of President Elect Joe Biden's inauguration this coming Tuesday."),
    };

    println!("From your news: {}", article.summarize());
    notify(&tweet);
}
