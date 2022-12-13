#![allow(dead_code)]
use serde::Deserialize;

mod archive;
mod media;
mod sql;
mod tweet;
pub use archive::TwitterArchive;
pub use media::{
    Media,
    Photo,
    Video,
};
pub use sql::{InsertSql, SelectStatusSql, Status, UpdateMediaSql};
pub use tweet::Tweet;

#[derive(Debug, Deserialize)]
pub struct TweetWrapper {
    pub tweet: Tweet,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Entities {
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<Url>,
    #[serde(default)]
    pub media: Vec<Media>,
    pub symbols: Vec<Symbol>,
    pub hashtags: Vec<Hashtag>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExtendedEntities {
    pub media: Vec<Media>
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserMention {
    name: String,
    screen_name: String,
    indicies: Option<[String; 2]>,
    id_str: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Url {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: [String; 2],
}

#[derive(Clone, Debug, Deserialize)]
pub struct Symbol {
    text: String,
    indices: [String; 2],
}

#[derive(Clone, Debug, Deserialize)]
pub struct Hashtag {
    text: String,
    indices: [String; 2],
}
