use std::error::Error;
use std::path::Path;
use chrono::NaiveDate;
use csv::{Reader, ReaderBuilder};

#[derive(Debug)]
pub struct Rat {
    pub id: u32,
    pub dob: NaiveDate,
    pub epoch: i64,
}

pub fn parse(file: &Path) -> Result<Vec<Rat>, Box<dyn Error>>
{
    let mut reader = Reader::from_path(file)?;

    let mut rats = Vec::new();

    for rat in reader.records() {
        let rat = rat?;
        let dob = NaiveDate::parse_from_str(rat.get(1).unwrap(), "%m/%d/%y")?;

        rats.push(Rat {
            id: rat.get(0).unwrap().parse()?,
            dob: dob.clone(),
            epoch: dob.and_hms_opt(0, 0, 0).unwrap().timestamp(),
        });
    }

    Ok(rats)
}