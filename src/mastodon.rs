use chrono::{DateTime, Utc, FixedOffset};
use native_tls::TlsConnector;
use postgres::Client as PostgresClient;
use postgres_native_tls::MakeTlsConnector;
use reqwest::{
    blocking::Client as ReqwestClient,
    blocking::multipart::Form,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::{env, error::Error, path::Path, thread::sleep, time::Duration};

pub struct MastodonClient {
    client: ReqwestClient,
    domain: String,
    sleep_duration: Duration,
}

impl MastodonClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let domain = env::var("MASTODON_DOMAIN")?;
        let token = env::var("MASTODON_TOKEN")?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))?,
        );
        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()?;

        let sleep_duration = Duration::from_secs(60);
        Ok(Self { client, domain, sleep_duration })
    }

    pub fn post_media(&mut self, media: &Path /* &Media */)
        -> Result<MediaResponse, Box<dyn Error>>
    {
        self.sleep();
        let url = format!("https://{}/api/v2/media", self.domain);
        let form = Form::new()
            .file("file", media)?;
        let res = self.client
            .post(&url)
            .multipart(form)
            .send()?;

        // no error propagation
        let _ = self.adjust_ratelimit(res.headers());

        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            println!("status: {status:?}");
            panic!("{}", res.text()?);
        }

        Ok(res.json()?)
    }

    fn sleep(&mut self) {
        println!("sleeping for: {:?}", self.sleep_duration);
        sleep(self.sleep_duration);
    }

    fn adjust_ratelimit(&mut self, headers: &HeaderMap)
        -> Result<(), Box<dyn Error>>
    {
        // Reset the timer, fallback for any early returns
        self.sleep_duration = Duration::from_secs(60);

        let remaining: u32 = headers.get("x-ratelimit-remaining")
            .ok_or("ratelimit remaining")?.to_str()?.parse()?;
        if remaining < 3 {
            let reset = headers.get("x-ratelimit-reset")
                .ok_or("ratelimit reset")?.to_str()?;
            let reset_at = DateTime::<FixedOffset>::parse_from_rfc3339(reset)?;

            let now = Utc::now();
            let duration = reset_at.signed_duration_since(now).to_std()?;
            self.sleep_duration = duration + Duration::from_secs(10);
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaResponse {
    id: String,
}

impl MediaResponse {
    pub fn id(&self) -> Result<i64, Box<dyn Error>> {
        Ok(self.id.parse()?)
    }
}

pub fn connect_postgres() -> Result<PostgresClient, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL")?;
    let connector = TlsConnector::new()?;
    let tls_connector = MakeTlsConnector::new(connector);

    Ok(PostgresClient::connect(&database_url, tls_connector)?)
}
