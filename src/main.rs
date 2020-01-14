mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod input;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        let day = args[1].parse::<u32>().expect("pick a day number");
        match day {
            1 => day1::run(),
            2 => day2::run(),
            3 => day3::run(),
            4 => day4::run(),
            5 => day5::run(),
            6 => day6::run(),
            7 => day7::run(),
            8 => day8::run(),
            _ => println!("unimplemented day"),
        }
    }
}
