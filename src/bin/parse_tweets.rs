#![allow(dead_code)]
use std::collections::BTreeMap;
use archive_parser::{
    TweetWrapper,
    Tweet,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // My archive is about 21M.
    // serde-rs/json#160 suggests that this into memory is best
    // https://github.com/serde-rs/json/issues/160
    let content = std::fs::read_to_string(".../data/tweets.js")?;

    let prefix = "window.YTD.tweets.part0 = ".len();
    let tweets: Vec<TweetWrapper> = serde_json::from_str(&content[prefix..])?;

    let mut my_tweets: BTreeMap<usize, &Tweet> = BTreeMap::new();

    for tweetw in &tweets {
        let tweet = &tweetw.tweet;
        my_tweets.insert(tweet.id(), tweet);
    }

    let mut max_tweet_id = 0;
    for (tweet_id, tweet) in my_tweets.clone().iter() {
        if *tweet_id > max_tweet_id {
            max_tweet_id = *tweet_id;
        } else {
            // Assume parents are always less than child tweet ids
            // If this tweet is a child tweet, we _must_ have seen it's parent
            // or it's parent does not exist / is not visible
            panic!("{} is less than {}", tweet_id, max_tweet_id);
        }

        if let Some(parent_tweet_id) = tweet.parent_id() {
            // We can't find a tweet to thread this onto
            // don't add this tweet to the archive
            if !my_tweets.contains_key(&parent_tweet_id) {
                my_tweets.remove(&tweet_id);
            }
        }
    };

    let mut tweet_count = 0;
    for (tweet_id, tweet) in my_tweets.iter_mut() {

        if tweet.is_retweet() {
            continue;
        }

        if tweet.references_deleted_tweet() {
            continue;
        }

        if let Some(parent_tweet_id) = tweet.parent_id() {
            println!("=== {tweet_id} in reply to {parent_tweet_id} ===");
        } else {
            println!("=== {tweet_id} ===");
        }
        println!("{tweet}");
        tweet_count += 1;

        if let Some(extended_entities) = &tweet.extended_entities {
            // Not media
            if extended_entities.media.is_empty() {
                break
            }

            // Not a photo
            for media in &extended_entities.media {
                if media.r#type() != "photo" {
                    break
                }
            }
        }
    }

    println!("Tweets: {tweet_count}");
    Ok(())
}
