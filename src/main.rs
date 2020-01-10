mod day1;
mod day2;
mod day3;
mod day4;
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
            _ => println!("unimplemented day"),
        }
    }
}
