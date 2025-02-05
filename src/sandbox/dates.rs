use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;

pub fn run() {
    // Utc date from local date and back to naive
    let tz: Tz = String::from("Europe/London").parse().unwrap();
    let date = NaiveDate::from_ymd_opt(2024, 05, 2).unwrap();
    let utc_date: DateTime<Utc> = tz
        .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        .unwrap()
        .with_timezone(&Utc);
    let local_naive = tz.from_utc_datetime(&utc_date.naive_utc()).date_naive();

    println!("{} {}", utc_date, local_naive);
}
