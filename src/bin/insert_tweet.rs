use serde::Deserialize;
use std::{
    env,
    error::Error,
    fs,
    path::PathBuf,
};
use archive_parser::TwitterArchive;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Debug, Deserialize)]
struct MediaResponse {
    id: String,
}

impl MediaResponse {
    fn id(&self) -> Result<u64, Box<dyn Error>> {
        Ok(self.id.parse()?)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mastodon_domain = env::var("MASTODON_DOMAIN")
        .expect("MASTODON_DOMAIN");
    let mastodon_token = env::var("MASTODON_TOKEN")
        .expect("MASTODON_TOKEN");
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {mastodon_token}"))?);
    let root_certificate = fs::read(...)?;
    let certificate = reqwest::Certificate::from_pem(&root_certificate)?;
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .add_root_certificate(certificate)
        .build()
        .expect("Build reqwest client");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL");
    let mut pg = postgres::Client::connect(&database_url, postgres::NoTls)
        .expect("Postgres Connect");

    let tweet_id = ...;

    let archive = TwitterArchive::from_dir(...)
        .expect("Load Twitter Archive Directory");
    let tweet = archive.get_tweet(tweet_id)
        .expect("Get Tweet");

    assert_eq!(tweet.id(), tweet_id);
    println!("{tweet}");

    let url = format!("https://{mastodon_domain}/api/v2/media");
    let mut media_attachment_ids: Vec<u64> = Vec::new();
    for media in tweet.media() {
        // XXX: base_dir should be built into the client
        let base_path = format!(
            "{}/data/tweets_media/{}",
            &archive.base_dir.display(),
            tweet.id(),
        );
        let file_name = media.archive_file_name(&base_path);
        println!("{file_name}");
        assert!(PathBuf::from(&file_name).exists());

        let form = reqwest::blocking::multipart::Form::new()
            .file("file", file_name)?;

        let res = client.post(&url)
            .multipart(form)
            .send()?;

        let status = res.status();
        let media_response: MediaResponse = res.json()?;
        let media_id: u64 = media_response.id()?;
        media_attachment_ids.push(media_id);

        println!("status: {status}");

   }

    println!("media_attachments: {:?}", media_attachment_ids);

    let sql = tweet.insert_sql()
        .with_media(&media_attachment_ids);

    let (query, values) = sql.as_query_values();
    pg.execute(query.as_str(), &values.as_params())?;

    for media_id in media_attachment_ids {
        let sql = archive_parser::UpdateMediaSql::default()
            .status_id(tweet.id() as u64)
            .media_id(media_id);
        let (query, values) = sql.as_query_values();
        pg.execute(query.as_str(), &values.as_params())?;
    }

    Ok(())
}
