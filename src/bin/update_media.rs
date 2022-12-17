use std::{
    env,
    error::Error,
};
use archive_parser::TwitterArchive;

fn main() -> Result<(), Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL")?;
    let mut pg = postgres::Client::connect(&database_url, postgres::NoTls)?;

    let tweet_id = ...;

    let archive = TwitterArchive::from_dir(...)
        .expect("Load Twitter Archive Directory");
    let tweet = archive.get_tweet(tweet_id)
        .expect("Get Tweet");

    assert_eq!(tweet.id(), tweet_id);

    let media_attachment_ids: Vec<i64> = vec![109491958253082956];

    for media_id in media_attachment_ids {
        let sql = archive_parser::UpdateMediaSql::default()
            .status_id(tweet.id())
            .media_id(media_id);
        let (query, values) = sql.as_query_values();

        println!("{query}");
        for param in values.as_params() {
            println!("  {param:?}");
        }

        pg.execute(query.as_str(), &values.as_params())?;
    }

    Ok(())
}
