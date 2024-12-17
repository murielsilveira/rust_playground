use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use rust_decimal::Decimal;
use rust_xlsxwriter::Workbook;

fn main() {
    let provider = get_provider("internal").unwrap();
    import_stuff(&provider);

    let _a = Decimal::new(10, 0);
    let _b = Decimal::new(11, 0);

    let x = 1267906.43;
    let y = 1267906.4299999997;
    let z: f64 = x - y;

    let _ = Workbook::default();
    // let _ = Workbook::new("tmp/test.xlsx").unwrap();

    println!("{} {}", z, z.abs() < 0.001);

    println!("default status {:?}", Status::default());

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

#[derive(Debug)]
enum Status {
    Open,
    Closed,
}

impl Status {
    fn default() -> Self {
        Status::Open
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Closed
    }
}

trait Provider {
    fn do_something(&self) -> String;
}

struct ExternalProvider;

struct InternalProvider;

impl Provider for ExternalProvider {
    fn do_something(&self) -> String {
        println!("External");
        String::from("External")
    }
}

impl Provider for InternalProvider {
    fn do_something(&self) -> String {
        println!("Internal");
        String::from("Internal")
    }
}

fn get_provider(provider_key: &str) -> Result<Box<dyn Provider>, String> {
    match provider_key {
        "external" => Ok(Box::new(ExternalProvider)),
        "internal" => Ok(Box::new(InternalProvider)),
        _ => Err(format!("Invalid provider [{provider_key}].")),
    }
}

fn import_stuff(provider: &Box<dyn Provider>) -> String {
    provider.do_something()
}
