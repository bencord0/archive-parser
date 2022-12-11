use chrono::NaiveDateTime;
use sea_query::{Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_postgres::{PostgresBinder, PostgresValues};

#[derive(Debug, Default)]
pub struct InsertSql {
    status_id: u64,
    account_id: u64,
    text: String,
    timestamp: NaiveDateTime,
    media_attachment_ids: Vec<u64>,
}

impl InsertSql {
    pub fn status_id(mut self, id: u64) -> Self {
        self.status_id = id;
        self
    }

    pub fn account_id(mut self, id: u64) -> Self {
        self.account_id = id;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn timestamp(mut self, timestamp: NaiveDateTime) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_media(mut self, media_attachment_ids: &[u64]) -> Self {
        self.media_attachment_ids = media_attachment_ids.to_vec();
        self
    }

    pub fn as_query_values(&self) -> (String, PostgresValues) {
        let mut columns = vec![
            Statuses::Id,
            Statuses::Uri,
            Statuses::Text,
            Statuses::CreatedAt,
            Statuses::UpdatedAt,
            Statuses::AccountId,
        ];

        let mut values: Vec<SimpleExpr> = vec![
            (self.status_id).into(),
            format!("https://twitter.com/{...}/status/{}", self.status_id).into(),
            self.text.clone().into(),
            self.timestamp.clone().into(),
            self.timestamp.clone().into(),
            (self.account_id).into(),
        ];

        if self.media_attachment_ids.len() > 0 {
            columns.push(Statuses::OrderedMediaAttachmentIds);
            values.push(self.media_attachment_ids.clone().into());
        }

        Query::insert()
            .into_table(Statuses::Table)
            .columns(columns)
            .values_panic(values)
            .build_postgres(PostgresQueryBuilder)
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
  OrderedMediaAttachmentIds,
}