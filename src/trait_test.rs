use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String;

    fn extract(&self) -> String {
        "some_data".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
        format!("{}: {}", self.username, self.content)
    }
}

impl <T:Display> Summary for T {
    fn summarize(&self) -> String {
        format!("{}: {}", self.to_string(), &self.to_string())
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "author".to_string(),
        content: "content".to_string(),
    };

    println!("New article: {}", news_article.summarize());
    println!("New article extracted: {}", news_article.extract());

    notify(&tweet);
    notify(&news_article);
}
