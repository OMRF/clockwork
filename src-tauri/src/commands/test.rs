use std::collections::HashMap;
use std::ops::Add;

use chrono::{Duration, Local, Months, NaiveDate, NaiveDateTime, TimeZone};
use tauri::api::dialog::blocking::FileDialogBuilder;
use z3::{ast::{Ast, Bool, Int}, Config, Context, SatResult, Solver};

use crate::util::rat_parser::parse;
use crate::util::serializer::{SacrificeDate, serialize};

#[tauri::command]
pub async fn test(
    allowed_days: Vec<i64>, // keep this in [0, 6]
    age_months: u32,
    tolerance_days: i64,
) -> Result<(), String> {
    let file = FileDialogBuilder::new()
        .add_filter("Input rats", &["csv"])
        .pick_file();

    let rats = match file {
        Some(file) => parse(file.as_path()).map_err(|_| "Failed to parse file".to_string())?,
        None => return Err("No file selected".to_string()),
    };

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // i could just combine this into 1 hashmap bit feeling a bit lazy
    // its also like 1 in the morning
    let mut sacrifice_epochs = Vec::new();
    let mut id_var_map = HashMap::new();

    for rat in rats.clone() {
        let target_age = rat.dob.checked_add_months(Months::new(age_months)).unwrap();
        let target_min = (target_age - Duration::days(tolerance_days)).and_hms_opt(0, 0, 0).unwrap().timestamp();
        let target_max = (target_age + Duration::days(tolerance_days)).and_hms_opt(0, 0, 0).unwrap().timestamp();

        let sacrifice_epoch = Int::new_const(&ctx, format!("rat_{}", rat.id));
        id_var_map.insert(sacrifice_epoch.clone(), rat.id);

        sacrifice_epochs.push(sacrifice_epoch.clone());

        solver.assert(&sacrifice_epoch.ge(&Int::from_i64(&ctx, target_min)));
        solver.assert(&sacrifice_epoch.le(&Int::from_i64(&ctx, target_max)));

        let days_since_epoch = sacrifice_epoch.div(&Int::from_i64(&ctx, 86400));
        let corrected_day_of_week = days_since_epoch
            .add(&Int::from_i64(&ctx, 3)) // Adjusting for the start day to align with the Unix epoch
            .modulo(&Int::from_i64(&ctx, 7));

        let allowed_days = allowed_days.iter().map(
            |day| corrected_day_of_week._eq(&Int::from_i64(&ctx, *day))
        ).collect::<Vec<_>>();
        let allowed_days_refs: Vec<&Bool> = allowed_days.iter().collect();

        solver.assert(&Bool::or(&ctx, &allowed_days_refs));
    }


    // Attempt to solve the model
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();

            let sacrifice_dates = sacrifice_epochs.iter().map(|rat_var| {
                let id = *id_var_map.get(rat_var).unwrap();
                let timestamp = model.eval(rat_var, true).unwrap().as_i64().unwrap();
                SacrificeDate {
                    rat_id: id.clone(),
                    dob: rats.iter().find(|rat| rat.id == id).unwrap().dob,
                    sacrifice_date: NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap().date(),
                }
            }).collect::<Vec<_>>();

            serialize(sacrifice_dates)?;
            return Ok(());
        }
        _ => Err("Could not find a viable schedule".to_string()),
    }?;

    Ok(())
}