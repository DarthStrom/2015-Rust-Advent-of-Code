use std::fs;

mod day1;
mod day2;

fn main() {
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("part1: {}", day1::get_floor(&contents));

    println!("part2: {}", day1::when_basement(&contents));
}
