use media_aggregator::{Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("john.doe"),
    content: String::from("lorem ipsum duo cquo luems."),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

}
