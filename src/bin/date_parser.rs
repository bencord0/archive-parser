use chrono::{DateTime, FixedOffset};
use std::{
    error::Error,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    let a = "2022-12-11T20:00:00.821685Z";
    let b = "2022-12-11T20:30:00.821685Z";

    //use chrono::Utc;
    //let da = Utc::now();
    let da = DateTime::<FixedOffset>::parse_from_rfc3339(a)
        .expect("parse da");

    let db = DateTime::<FixedOffset>::parse_from_rfc3339(b)
        .expect("parse db");

    let delta: Duration = db.signed_duration_since(da).to_std()
        .expect("delta");
    println!("delta: {delta:?}");

    Ok(())
}
