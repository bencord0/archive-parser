use clap::Parser;
use archive_parser::{SelectStatusSql, TwitterArchive, connect_postgres};
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(author, version)]
struct Config {
    #[arg(long)]
    archive: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let mut pg = connect_postgres()?;

    let archive = TwitterArchive::from_dir(config.archive)?;
    for (tweet_id, tweet) in archive.tweets.iter() {
        let sql = SelectStatusSql::default().status_id(*tweet_id);
        let status = sql.fetch(&mut pg)?;

        if tweet == status {
            println!("{status}");
            continue;
        }

        println!("tweet: {tweet}");

        let sql = tweet.insert_sql();
        let (query, values) = sql.as_query_values();
        pg.execute(query.as_str(), &values.as_params())?;
    }
    Ok(())
}
