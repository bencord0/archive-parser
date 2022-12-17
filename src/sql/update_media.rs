use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_postgres::{PostgresBinder, PostgresValues};

#[derive(Debug, Default)]
pub struct UpdateMediaSql {
    status_id: i64,
    media_attachment_id: i64,
}

impl UpdateMediaSql {
    pub fn status_id(mut self, id: i64) -> Self {
        self.status_id = id;
        self
    }

    pub fn media_id(mut self, media_attachment_id: i64) -> Self {
        self.media_attachment_id = media_attachment_id;
        self
    }

    pub fn as_query_values(&self) -> (String, PostgresValues) {
        let expr = Expr::col(MediaAttachments::Id).eq(self.media_attachment_id);
        Query::update()
            .table(MediaAttachments::Table)
            .values([(MediaAttachments::StatusId, self.status_id.into())])
            .and_where(expr)
            .build_postgres(PostgresQueryBuilder)
    }
}

#[derive(Iden)]
enum MediaAttachments {
    Table,
    Id,
    StatusId,
}
