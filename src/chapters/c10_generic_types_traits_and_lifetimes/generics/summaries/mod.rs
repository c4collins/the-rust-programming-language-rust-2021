use std::fmt::{Debug, Display, Formatter, Result};

pub trait Summary {
    // This trait method has signature but no default implementation
    fn summarize_author(&self) -> String;

    // This trait method has an overrideable default (the signature is permanent)
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}
impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize()); // just using Summary::summarize()
}

pub fn notify_without_sugar<T: Summary + Display>(item: &T) {
    println!("Breaking news! {} {}", item, item.summarize()); // Using Display trait and Summary::summarize() from trait
}

pub fn using_where<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}, {:?}", t, u);
}

// returning a trait type
pub fn returning_a_tweet() -> impl Summary {
    Tweet {
        username: String::from("c4collins"),
        content: String::from("Hello"),
        reply: true,
        retweet: false,
    }
}
