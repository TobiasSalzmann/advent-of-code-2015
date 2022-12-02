mod day1;
mod util;
mod day2;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    let day_string = env::args().nth(1).or_else(|| {
        dotenv().ok();
        env::var("DAY").ok()
    }).unwrap_or("1".to_string());

    let day = day_string.parse::<i32>().expect("Wrong format for day variable");

    match day {
        1 => day1::main(),
        2 => day2::main(),
        _ => {println!("Not yet implemented 😅")}
    }
}
