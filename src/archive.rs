use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::PathBuf,
};
use crate::{TweetWrapper, Tweet};

pub struct TwitterArchive {
    pub base_dir: PathBuf,
    pub tweets: BTreeMap<usize, Tweet>,
}

impl TwitterArchive {
    pub fn from_dir<P>(base_dir: P) -> Result<Self, Box<dyn Error>>
    where
        P: Into<PathBuf>,
    {
        let base_dir = base_dir.into();
        let tweets = BTreeMap::new();
        let mut archive = Self {
            base_dir,
            tweets,
        };

        archive.init()?;
        Ok(archive)
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut tweet_path = self.base_dir.clone();
        tweet_path.push("data/tweets.js");
        let tweet_content = fs::read_to_string(tweet_path)?;
        let prefix = "window.YTD.tweets.part0 = ".len();
        let tweets: Vec<TweetWrapper> = serde_json::from_str(&tweet_content[prefix..])?;

        for tweetw in tweets {
            let tweet = tweetw.tweet;
            self.tweets.insert(tweet.id(), tweet);
        }

        // Remove unreachable tweets
        // We assume that tweet ids are in chronological order.
        // We also rely on BTreeMap iteration working on sorted data.
        for (tweet_id, tweet) in self.tweets.clone().iter() {
            if let Some(parent_tweet_id) = tweet.parent_id() {
                // We can't find a tweet to thread this onto
                // don't make this tweet available to the archive
                if !self.tweets.contains_key(&parent_tweet_id) {
                    self.tweets.remove(tweet_id);
                    continue;
                }
            }

            if tweet.is_retweet() {
                self.tweets.remove(tweet_id);
                continue;
            }

            if tweet.references_deleted_tweet() {
                self.tweets.remove(tweet_id);
                continue;
            }
        }

        Ok(())
    }

    pub fn get_tweet(&self, id: usize) -> Result<&Tweet, Box<dyn Error>> {
        self.tweets.get(&id).ok_or_else(|| "not found".into())
    }
}
