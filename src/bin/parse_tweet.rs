use archive_parser::TwitterArchive;

fn main() {
    let tweet_id = ...;

    let archive = TwitterArchive::from_dir(...)
        .expect("Load Twitter Archive Directory");
    let tweet = archive.get_tweet(tweet_id)
        .expect("Get Tweet");

    println!("{tweet}");
}
