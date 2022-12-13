use std::{env, error::Error};

fn main() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL");

    let connector = native_tls::TlsConnector::new()
        .expect("native_tls::TlsConnector");

    let tls_connector = postgres_native_tls::MakeTlsConnector::new(connector);

    let mut pg = postgres::Client::connect(&database_url, tls_connector)
        .expect("postgres connect");

    let result = pg.execute("SELECT 1", &[])
        .expect("SELECT 1");

    println!("{result}");
}
