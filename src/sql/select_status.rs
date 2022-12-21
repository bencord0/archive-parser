use chrono::NaiveDateTime;
use postgres::Row;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_postgres::{PostgresBinder, PostgresValues};
use std::error::Error;

#[derive(Debug, Default)]
pub struct SelectStatusSql {
    status_id: i64,
}

impl SelectStatusSql {
    pub fn status_id(mut self, id: i64) -> Self {
        self.status_id = id;
        self
    }

    pub fn as_query_values(&self) -> (String, PostgresValues) {
        let columns = vec![
            Statuses::Id,
            Statuses::Uri,
            Statuses::Text,
            Statuses::CreatedAt,
            Statuses::UpdatedAt,
            Statuses::AccountId,
            Statuses::InReplyToId,
            Statuses::OrderedMediaAttachmentIds,
        ];

        Query::select()
            .from(Statuses::Table)
            .columns(columns)
            .and_where(Expr::col(Statuses::Id).eq(self.status_id))
            .limit(1)
            .build_postgres(PostgresQueryBuilder)
    }

    pub fn fetch(&self, pg: &mut postgres::Client) -> Result<Status, Box<dyn Error>> {
        let (query, values) = self.as_query_values();
        let row = pg.query_one(query.as_str(), &values.as_params())?;
        Ok(row.into())
    }
}

#[derive(Iden)]
enum Statuses {
    Table,
    Id,
    Uri,
    AccountId,
    Text,
    CreatedAt,
    UpdatedAt,
    InReplyToId,
    OrderedMediaAttachmentIds,
}

#[derive(Debug, Default)]
pub struct Status {
    pub id: i64,
    pub uri: String,
    pub account_id: i64,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub in_reply_to_id: Option<i64>,
    pub ordered_media_attachment_ids: Option<Vec<i64>>,
}

impl Status {
    pub fn media_count(&self) -> usize {
        if let Some(media_ids) = &self.ordered_media_attachment_ids {
            return media_ids.len();
        }

        0
    }
}

impl From<Row> for Status {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            uri: row.get("uri"),
            account_id: row.get("account_id"),
            text: row.get("text"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            in_reply_to_id: row.get("in_reply_to_id"),
            ordered_media_attachment_ids: row.get("ordered_media_attachment_ids"),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id   : {}\n", self.id)?;
        write!(f, "text : {}\n", self.text)?;
        if let Some(media_ids) = &self.ordered_media_attachment_ids {
            for media_id in media_ids {
                write!(f, "media: {}\n", media_id)?;
            }
        }

        Ok(())
    }
}

use crate::Tweet;
impl PartialEq<Status> for &Tweet {
    fn eq(&self, other: &Status) -> bool {
        let tweet_id: i64 = self.id().try_into().unwrap();
        tweet_id == other.id
        && format!("https://twitter.com/bencord0/status/{}", tweet_id) == other.uri
        && self.replaced_text() == other.text
        && self.created_at == other.created_at
        && self.created_at == other.updated_at
        && self.parent_id().map(|id| id as i64) == other.in_reply_to_id
        && self.media().len() == other.media_count()
    }
}
