pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// another way
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// // This way, we can guarantee that item 1 and item 2 oare of type T
// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarizale() -> impl Summary{
    Tweet{
        username: String::from("Housr_hook"),
        content: String::from("Of course"),
        reply:false,
        retweet:false
    }
}

struct Pair<T>{
    x:T,
    y:T,
}

impl<T> Pair<T>{
    fn new(x:T, y:T) -> Self{
        Self{x,y}
    }
}
trait Display {
    //..
}

// Blinking trait, you can build a trait on top of a trait: more on this later :)
// impl <T: Display> ToString for T{}


fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky s actally failling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    notify(&tweet);
    returns_summarizale();
}
