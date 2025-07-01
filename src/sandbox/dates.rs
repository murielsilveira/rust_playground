use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
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

    let now = Utc::now().date_naive().and_hms_opt(23, 5, 0).unwrap();
    let t1 = now.format("%Y%m%d").to_string();
    let t2 = now
        .and_local_timezone(Utc)
        .unwrap()
        .with_timezone(&chrono_tz::Europe::London)
        .format("%Y%m%d")
        .to_string();

    println!("UTC: {} London: {}", t1, t2);

    println!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%S.%3f"));
    println!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f"));
    println!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%S.%3f"));
    println!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%S.%f")); // give 9 "decimals"

    println!("{:?}", parse_ts("2025-10-10 10:10:10.12345678901234"));
    println!("{:?}", parse_ts("2025-10-10 10:10:10"));
}

pub fn parse_ts(date: &str) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S%.f").unwrap(),
        Utc,
    )
}
