fn main() {
    println!("Hello world!");

    let tweet = Tweet {
        username: String::from("@john"),
        content: String::from("Hello John"),
        reply: false,
        retweet: false,
    };

    let article = News {
        author: String::from("@notjohn"),
        headline: String::from("Goodby John"),
        content: String::from("..."),
    };

    println!("{}", tweet.sumarize());
    println!("{}", article.sumarize());
}

pub struct News {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for News {
    fn sumarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn sumarize(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.sumarize());
}
