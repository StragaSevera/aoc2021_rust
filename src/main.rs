use std::env::args;

fn main() {
    let day = args().nth(1).expect("Need to pass the day number!");
    let result = match day.as_str() {
        "01a" => day01a::result(),
        "01b" => day01b::result(),
        "02a" => day02a::result(),
        "02b" => day02b::result(),
        "03a" => day03a::result(),
        "03b" => day03b::result(),
        _ => panic!("The day number should be a valid implemented day with two digits with suffix specifying 'a' or 'b'!")
    };
    println!("Result for day {}: {}", day, result);
}

mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
