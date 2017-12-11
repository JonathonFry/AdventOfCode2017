mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

extern crate time;
extern crate regex;
use self::time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

//    day1::solution();
//    day2::solution();
//    day3::solution();
//    day4::solution();
//    day5::solution();
//    day6::solution();
//    day7::solution();
//    day8::solution();
    day9::solution();

    let end = PreciseTime::now();
    println!("{} ms", start.to(end).num_milliseconds());
}
