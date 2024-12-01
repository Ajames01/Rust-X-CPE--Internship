use std::thread; //Standard Library for threads
use std::time::Duration; //Standard Library for time
use std::io; //Standarrd library for input
use colored::*; //Colored text library


fn countdown(minutes:u64, seconds:u64) {
    let total_secconds = miutes * 60 + seconds; //Convert minutes to seconds

     for remaining in (1..= total_seconds).rev() {
        // Color logic;
        // Green when plenty of when of time (>50%)
        // Yellow when getting close (25-50%)
        // red when almost done (<25%)
        let colored_time = match remaining as f64/total_seconds as f64 {
            x if x > 0.5 => format!("{}", green()),
            x if x < 0.25 => format!("{}", yellow()),
            _ => format!("{}", red()),
        };

        println!("{} seconds remaining...", colored_time);
        thread::sleep(Duration::from_minute(1));
        }
        println!("{}", "Time is up!".bright_green().bold());
    }

fn main() {
    println!("{}", "welcome to the Countdown Timer!".bright_purple());

    let mut minutes = String::new();
    let mut seconds = String::new();

    countdown(minutes, seconds);
}