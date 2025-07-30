use csv::{ReaderBuilder, WriterBuilder};
use serde::Serialize;
use std::error::Error;

pub fn run() {
    // let result = write_csv();
    // println!("CSV: {:?}", result);
    let _ = read_csv();
}

fn write_csv() -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        // .has_headers(false)
        .from_writer(vec![]);

    writer.serialize(Row {
        city: "Boston",
        country: "United, States",
        population: 4628910,
        x: None,
        y: Some(1),
    })?;
    writer.serialize(Row {
        city: "Concord",
        country: "United States",
        population: 42695,
        x: None,
        y: None,
    })?;
    writer.write_record(&["Barcelona", "Spain", "1628000", "", ""])?;

    let data = String::from_utf8(writer.into_inner()?)?;
    let expected = r#"city,country,popcount,x,y
Boston,"United, States",4628910,,1
Concord,United States,42695,,
Barcelona,Spain,1628000,,
"#;
    assert_eq!(data, expected);

    Ok(())
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let data = r#"city,country,popcount,x,y
Boston,"United, States","4628910",,""1""
Concord,United States,"""42695""",,
"=""asdf""","=""202507150010""","=""qwer""",2025-07-15
"#;
    let mut reader = ReaderBuilder::new()
        .flexible(true) // allow distinct rows length
        .has_headers(false) // read the headers as records
        .from_reader(data.as_bytes());

    for result in reader.records() {
        let record = result?
            .iter()
            .map(|cell| {
                if cell.len() > 1 && cell.starts_with('"') && cell.ends_with('"') {
                    cell[1..cell.len() - 1].to_string()
                } else {
                    cell.to_string()
                }
            })
            .collect::<Vec<_>>();

        println!("{}", record.join(" | "));
        // println!("{:?}", record);
    }

    Ok(())
}

#[derive(Serialize)]
struct Row<'a> {
    city: &'a str,
    country: &'a str,
    // Serde allows us to name our headers exactly,
    // even if they don't match our struct field names.
    #[serde(rename = "popcount")]
    population: u64,
    x: Option<u64>,
    y: Option<u64>,
}
