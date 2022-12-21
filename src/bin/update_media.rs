use clap::Parser;
use archive_parser::{TwitterArchive, connect_postgres};
use std::{
    error::Error,
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version)]
struct Config {
    #[arg(long, help = "Path to unpacked twitter archive")]
    archive: PathBuf,
    #[arg(long, help = "Twitter status id")]
    tweet_id: i64,
    #[arg(long, help = "Print SQL query only")]
    dry_run: bool,
    media_attachment_ids: Vec<i64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let mut pg = connect_postgres()?;

    let archive = TwitterArchive::from_dir(config.archive)?;
    let tweet = archive.get_tweet(config.tweet_id)?;

    assert_eq!(tweet.id(), config.tweet_id);

    for media_id in config.media_attachment_ids {
        let sql = archive_parser::UpdateMediaSql::default()
            .status_id(tweet.id())
            .media_id(media_id);
        let (query, values) = sql.as_query_values();

        println!("{query}");
        for param in values.as_params() {
            println!("  {param:?}");
        }

        if !config.dry_run {
            pg.execute(query.as_str(), &values.as_params())?;
        }
    }

    Ok(())
}
