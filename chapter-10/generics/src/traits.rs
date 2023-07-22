use crate::summaries::{
    notify, notify_without_sugar, returning_a_tweet, NewsArticle, Summary, Tweet,
};

pub fn go() {
    let tweet = Tweet {
        username: String::from("c4collins"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    let tweet2 = returning_a_tweet();

    notify(&tweet2);
    notify_without_sugar(&article);
}
