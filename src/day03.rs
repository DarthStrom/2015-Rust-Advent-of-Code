use crate::input;
use std::collections::HashMap;

pub fn run() {
    let contents = input::get_contents("day03");

    println!("part1: {}", count_houses(&contents));
    println!("part2: {}", count_houses_dual(&contents));
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct V2 {
    x: i32,
    y: i32,
}

impl Copy for V2 {}

fn count_houses(directions: &str) -> usize {
    let mut current_location = V2 { x: 0, y: 0 };
    let mut houses: HashMap<V2, i32> = HashMap::new();
    houses.insert(current_location, 1);

    for c in directions.chars() {
        match c {
            '>' => current_location.x += 1,
            'v' => current_location.y += 1,
            '<' => current_location.x -= 1,
            '^' => current_location.y -= 1,
            _ => panic!("unknown direction"),
        }
        let entry = houses.entry(current_location);
        *entry.or_insert(1) += 1;
    }

    houses.keys().count()
}

fn count_houses_dual(directions: &str) -> usize {
    let mut santa_location = V2 { x: 0, y: 0 };
    let mut robo_location = V2 { x: 0, y: 0 };
    let mut houses: HashMap<V2, i32> = HashMap::new();
    houses.insert(santa_location, 2);

    for (i, c) in directions.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '>' => santa_location.x += 1,
                'v' => santa_location.y += 1,
                '<' => santa_location.x -= 1,
                '^' => santa_location.y -= 1,
                _ => panic!("unknown direction"),
            }
            let entry = houses.entry(santa_location);
            *entry.or_insert(1) += 1;
        } else {
            match c {
                '>' => robo_location.x += 1,
                'v' => robo_location.y += 1,
                '<' => robo_location.x -= 1,
                '^' => robo_location.y -= 1,
                _ => panic!("unknown direction"),
            }
            let entry = houses.entry(robo_location);
            *entry.or_insert(1) += 1;
        }
    }

    houses.keys().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(count_houses(">"), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_houses("^>v<"), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_houses("^v^v^v^v^v"), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(count_houses_dual("^v"), 3);
    }

    #[test]
    fn example5() {
        assert_eq!(count_houses_dual("^>v<"), 3);
    }

    #[test]
    fn example6() {
        assert_eq!(count_houses_dual("^v^v^v^v^v"), 11);
    }
}
