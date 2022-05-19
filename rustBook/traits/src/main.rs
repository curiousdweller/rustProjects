#[derive(Debug)]
enum TweetStatus {
    Tweet,
    ReTweet,
    Reply
}

struct Tweet {
    status: TweetStatus,
    username: String,
    content: String

}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("1 new {:#?} from {}: {}", self.status, self.username, self.content)
    }
}


struct NewsPaper {
    headline: String,
    content: String,
    author: String
}

impl Summary for NewsPaper {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
trait Summary {
    fn summarize(&self) -> String;
}
fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        status: TweetStatus::Tweet,
        username: "LiLkiid".to_string(),
        content: "LFG".to_string()
    };
    println!("{}", tweet.summarize());

}


