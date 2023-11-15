use cron::Schedule;
use chrono::{Utc, Duration};

// daily_cron accepts a function that executes daily
async fn daily_cron(func: fn()) {
    // Define the cron schedule for daily execution
    let schedule = Schedule::from_str("0 3 * * *").expect("Invalid cron schedule");

    // Schedule the daily cron to run indefinitely
    loop {
        let now = Utc::now();
        let next_execution = schedule.upcoming(Utc).next().unwrap();
        let duration_left = next_execution - now;

        // Sleep 
        time::sleep(TokioDuration::from_std(duration_left.to_std().unwrap())).await;

        // Execute the job
        func().await;
    }
}
