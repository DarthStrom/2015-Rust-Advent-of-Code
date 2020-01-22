mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod input;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        let day = args[1]
            .replace("day", "")
            .parse::<u32>()
            .expect("pick a day number");
        match day {
            1 => day1::run(),
            2 => day2::run(),
            3 => day3::run(),
            4 => day4::run(),
            5 => day5::run(),
            6 => day6::run(),
            7 => day7::run(),
            8 => day8::run(),
            9 => day9::run(),
            10 => day10::run(),
            11 => day11::run(),
            12 => day12::run(),
            13 => day13::run(),
            14 => day14::run(),
            15 => day15::run(),
            16 => day16::run(),
            _ => println!("unimplemented day"),
        }
    } else {
        println!("Which day?");
    }
}
