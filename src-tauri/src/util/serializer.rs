use chrono::NaiveDate;
use tauri::api::dialog::blocking::FileDialogBuilder;

pub struct SacrificeDate {
    pub rat_id: i64,
    pub dob: NaiveDate,
    pub sacrifice_date: NaiveDate
}

pub fn serialize(rats: Vec<SacrificeDate>) -> Result<(), String> {
    let path = FileDialogBuilder::new()
        .add_filter("Output schedule", &["csv"])
        .set_file_name("sacrifice_schedule.csv")
        .save_file();
    let path = match path {
        Some(path) => path,
        None => return Err("Save aborted".to_string()),
    };

    let mut wtr = csv::Writer::from_path(path).map_err(|_| "Failed to write file".to_string())?;

    wtr.write_record(&["Rat Number", "Date of Birth", "Sacrifice Date"]).map_err(|_| "Failed to write file".to_string())?;

    for rat in rats {
        wtr.write_record(&[rat.rat_id.to_string(), rat.dob.to_string(), rat.sacrifice_date.to_string()]).map_err(|_| "Failed to write file".to_string())?;
    }

    wtr.flush().map_err(|_| "Failed to write file".to_string())?;

    Ok(())
}