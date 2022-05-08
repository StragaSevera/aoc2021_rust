use std::env::args;

fn main() {
    let day = args().nth(1).expect("Need to pass the day number!");
    let result = match day.as_str() {
        "01a" => lesson01a::result(),
        "01b" => lesson01b::result(),
        _ => panic!("The day number should be a valid implemented day with two digits with suffix specifying 'a' or 'b'!")
    };
    println!("Result for day {}: {}", day, result);
}

mod lesson01a;
mod lesson01b;