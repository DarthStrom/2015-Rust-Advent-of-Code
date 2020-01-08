mod day1;
mod day2;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        let day = args[1].parse::<u32>().expect("pick a day number");
        match day {
            1 => day1::run(),
            2 => day2::run(),
            _ => println!("unimplemented day"),
        }
    }
}
