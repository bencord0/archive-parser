#![allow(dead_code)]
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{
    Entities,
    ExtendedEntities,
    InsertSql,
};

use crate::media::{
    Media,
    Photo,
    Video,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Tweet {
    pub id_str: String,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub entities: Entities,
    pub display_text_range: [String; 2],
    pub favorite_count: String,
    pub favorited: bool,
    pub retweet_count: String,
    pub retweeted: bool,
    pub truncated: bool,
    #[serde(default)]
    pub possibly_sensitive: bool,
    #[serde(with = "twitter_date_format")]
    pub created_at: NaiveDateTime,
    pub full_text: String,
    pub extended_entities: Option<ExtendedEntities>,
}

impl Tweet {
    pub fn id(&self) -> i64 {
        self.id_str.parse().unwrap()
    }

    pub fn parent_id(&self) -> Option<i64> {
        match &self.in_reply_to_status_id_str {
            Some(id) => Some(id.parse().unwrap()),
            None => None,
        }
    }

    pub(crate) fn replaced_text(&self) -> String {
        let mut text = self.full_text.to_string();

        for url in &self.entities.urls {
            text = text.replace(&url.url, &url.expanded_url);
        }

        for mention in &self.entities.user_mentions {
            let screen_name = &mention.screen_name;
            let original = format!("@{screen_name}");
            let replacement = format!("@{screen_name}@twitter.com");
            text = text.replace(&original, &replacement);
        }

        for media in &self.entities.media {
            let url = match media {
                // No leading whitespace
                Media::Photo(Photo{url, indices: Some([index, _]), ..}) if index == "0" => url.clone(),
                // Media is in the middle of the tweet, also capture a leading space
                Media::Photo(Photo{url, ..}) => {
                    let mut u = url.clone();
                    u.insert(0, ' ');
                    u
                },
                _ => {
                    panic!("unrecognised media type");
                },
            };

            text = text.replace(&url, "");
        }

        text
    }

    pub fn media(&self) -> Vec<Media> {
        match &self.extended_entities {
            Some(entities) => entities.media.clone(),
            None => Vec::new(),
        }
    }

    pub fn is_retweet(&self) -> bool {
        let mut markers: Vec<String> = Vec::new();
        for user_mention in &self.entities.user_mentions {
            let screen_name = &user_mention.screen_name;
            let marker = format!("RT @{screen_name}:");
            markers.push(marker);
        }

        markers.iter().any(|m| self.full_text.starts_with(m))
    }

    pub fn references_deleted_tweet(&self) -> bool {
        self.replaced_text().contains("https://t.co/")
    }

    pub fn insert_sql(&self) -> InsertSql {
        let mut sql = InsertSql::default()
            .status_id(self.id())
            .account_id(109399109809644293)
            .text(self.replaced_text())
            .timestamp(self.created_at.clone());

        if let Some(in_reply_to_id) = self.parent_id() {
            sql = sql.in_reply_to_id(in_reply_to_id);
        }

        sql
    }
}

impl std::fmt::Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let replaced_text = self.replaced_text();
        write!(f, "id: {}\n", self.id_str)?;
        if let Some(in_reply_to) = &self.in_reply_to_status_id_str {
            write!(f, "in_reply_to: {}\n", in_reply_to)?;
        }
        write!(f, "{replaced_text}\n")?;

        write!(f, "  {}\n", self.created_at)?;
        if let Some(extended_entities) = &self.extended_entities {
            for media in &extended_entities.media {
                let r#type = media.r#type();
                let file_name = media.archive_file_name(&self.id_str);
                write!(f, "  {type}| {file_name}\n")?;
            }
        }

        Ok(())
    }
}

mod twitter_date_format {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer};
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, "%a %b %d %T %z %Y")
            .map_err(serde::de::Error::custom)
    }
}
