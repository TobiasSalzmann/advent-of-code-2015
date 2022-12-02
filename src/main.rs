mod day1;
mod util;
mod day2;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {

    dotenv().ok();
    let day_string = env::var("DAY").unwrap_or("1".to_string());
    let day = day_string.parse::<i32>().expect("Wrong format for day variable");

    println!("Hello, day {}!", day);

    match day {
        1 => day1::main(),
        2 => day2::main(),
        _ => {}
    }
}
