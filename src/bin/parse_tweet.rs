use archive_parser::TwitterArchive;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version)]
struct Config {
    #[arg(long)]
    archive: PathBuf,
    #[arg(long)]
    tweet: usize,
}

fn main() {
    let config = Config::parse();

    let archive = TwitterArchive::from_dir(config.archive)
        .expect("Load Twitter Archive Directory");

    let tweet = archive.get_tweet(config.tweet)
        .expect("Get Tweet");

    println!("{tweet}");
}
