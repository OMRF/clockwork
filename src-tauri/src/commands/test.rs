use std::error::Error;
use std::ops::Add;
use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Weekday};
use tauri::api::dialog::blocking::FileDialogBuilder;
use z3::{ast, Tactic, ast::{Array, Ast, AstKind, Bool, Dynamic, Float, Int, Real, BV}, Config, Context, DeclKind, FuncDecl, Sort, Solver, SatResult};
use crate::util::rat_parser::parse;

#[tauri::command]
pub async fn test() -> Result<(), String> {
    let file_path = FileDialogBuilder::new()
        .add_filter("Input Rats", &["csv"])
        .pick_file();

    let rats = parse(file_path.unwrap().as_path()).unwrap();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let mut rat_sacrifice_epochs = Vec::new();

    for rat in rats {
        let rat_age_24_months = rat.epoch + 24 * 30 * 24 * 60 * 60;
        let rat_sacrifice_min = rat_age_24_months - 10 * 24 * 60 * 60;
        let rat_sacrifice_max = rat_age_24_months + 10 * 24 * 60 * 60;

        let rat_sacrifice_epoch = Int::new_const(&ctx, format!("rat_{}", rat.id));

        rat_sacrifice_epochs.push(rat_sacrifice_epoch.clone());

        solver.assert(&rat_sacrifice_epoch.ge(&Int::from_i64(&ctx, rat_sacrifice_min)));
        solver.assert(&rat_sacrifice_epoch.le(&Int::from_i64(&ctx, rat_sacrifice_max)));

        let days_since_epoch = rat_sacrifice_epoch.div(&Int::from_i64(&ctx, 86400));
        let corrected_day_of_week = days_since_epoch
            .add(&Int::from_i64(&ctx, 3)) // Adjusting for the start day to align with the Unix epoch
            .modulo(&Int::from_i64(&ctx, 7));

        let not_weekend = Bool::and(&ctx, &[
            &corrected_day_of_week.lt(&Int::from_i64(&ctx, 5)), // Check if day is before Saturday
            &corrected_day_of_week.gt(&Int::from_i64(&ctx, 0)), // Optionally, ensure it's also after Sunday (useful if your week starts on Sunday)
        ]);

        solver.assert(&not_weekend);
    }


    // Attempt to solve the model
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            for rat_var in rat_sacrifice_epochs {
                if let Some(timestamp) = model.eval(&rat_var, true) {
                    // use chrono to pretty print the timestamp
                    let timestamp = timestamp.as_i64().unwrap();
                    let sacrifice_date = Local.timestamp_opt(timestamp, 0).unwrap();

                    println!("Rat {} should be sacrificed on {}", rat_var, sacrifice_date.format("%A, %B %e, %Y"));
                }
            }
        }
        _ => println!("Could not find a solution with the given constraints."),
    }

    Ok(())
}


// let rat_age_24_months = rat.dob + Duration::days(24 * 30);
// let rat_sacrifice_min = rat_age_24_months - Duration::days(10);
// let rat_sacrifice_max = rat_age_24_months + Duration::days(10);
//
// // Create a variable for each rat's sacrifice day within the experiment timeframe
// // For simplicity, we represent days as integers from the start of the experiment period
// let rat_var = Int::new_const(&ctx, format!("rat_{}", rat.id));
// rat_sacrifice_days.push(rat_var.clone());
//
// // Constraint: rat sacrifice day must be within 24 months ± 10 days of its dob, represented in experiment period days
// solver.assert(&rat_var.ge(&Int::from_i64(&ctx, rat_sacrifice_min.signed_duration_since(experiment_start).num_days())));
// solver.assert(&rat_var.le(&Int::from_i64(&ctx, rat_sacrifice_max.signed_duration_since(experiment_start).num_days())));