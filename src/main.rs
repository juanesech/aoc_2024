use std::env;
mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let day: &str = &args[2];

    println!("Day: {:?}, input: {:?}", day, input_file);

    match day {
        "day1" => libs::day_1::run(input_file),
        "day2" => libs::day_2::run(input_file),
        "day3" => libs::day_3::run(input_file, libs::day_3::Part::P1),
        _ => day_err(),
    };
    println!("Done")
}

fn day_err() -> i32 {
    println!("Day not available");
    0
}
