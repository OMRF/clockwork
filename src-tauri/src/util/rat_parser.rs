use std::error::Error;
use std::path::Path;

use chrono::NaiveDate;
use csv::Reader;

#[derive(Debug, Clone)]
pub struct Rat {
    pub id: i64,
    pub dob: NaiveDate,
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
            dob,
        });
    }

    Ok(rats)
}