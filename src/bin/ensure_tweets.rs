use clap::Parser;
use archive_parser::{MastodonClient, SelectStatusSql, TwitterArchive, connect_postgres};
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(author, version)]
struct Config {
    #[arg(long)]
    archive: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let mut mastodon = MastodonClient::new()?;
    let mut pg = connect_postgres()?;

    let archive = TwitterArchive::from_dir(config.archive)?;
    for (tweet_id, tweet) in archive.tweets.iter() {
        let sql = SelectStatusSql::default().status_id((*tweet_id));
        let status = sql.fetch(&mut pg).unwrap_or_default();
        if tweet == status {
            println!("{status}");
            continue;
        }

        println!("tweet: {tweet:?}");
        println!("status: {status:?}");

        // --- Need to insert the tweet ---
        let mut media_attachment_ids: Vec<i64> = Vec::new();
        let tweet_media = tweet.media();
        if tweet_media.len() != status.media_count() {
            for media in tweet_media {
                // XXX: base_dir should be built into the client
                let base_path = format!(
                    "{}/data/tweets_media/{}",
                    &archive.base_dir.display(),
                    tweet.id(),
                );
                let file_name = PathBuf::from(&media.archive_file_name(&base_path));
                println!("{file_name:?}");
                assert!(file_name.exists());

                let media_response = mastodon.post_media(&file_name)?;
                let media_id: i64 = media_response.id()?;
                media_attachment_ids.push(media_id);
            }
        } else {
            if let Some(media_ids) = &status.ordered_media_attachment_ids {
                media_attachment_ids = media_ids.clone();
            }
        }

        println!("media_attachments: {:?}", media_attachment_ids);

        let sql = tweet.insert_sql().with_media(&media_attachment_ids);
        let (query, values) = sql.as_query_values();
        pg.execute(query.as_str(), &values.as_params())?;

        for media_id in media_attachment_ids {
            let sql = archive_parser::UpdateMediaSql::default()
                .status_id(tweet.id())
                .media_id(media_id);
            let (query, values) = sql.as_query_values();
            pg.execute(query.as_str(), &values.as_params())?;
        }
    }
    Ok(())
}
