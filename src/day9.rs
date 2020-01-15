use crate::input::get_lines;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn run() {
    let lines = get_lines("day9");

    println!("part1: {}", shortest(&lines));
}

fn shortest(lines: &[String]) -> usize {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse::<usize>().unwrap();
        paths.insert((from, to), distance);
        paths.insert((to, from), distance);
        cities.insert(from);
        cities.insert(to);
        paths.insert(("start", from), 0);
        paths.insert(("start", to), 0);
    }

    solve(&cities, &paths, "start", HashSet::new())
}

fn solve(
    cities: &HashSet<&str>,
    paths: &HashMap<(&str, &str), usize>,
    current: &str,
    visited: HashSet<&str>,
) -> usize {
    if visited.len() == cities.len() {
        0
    } else {
        let mut min = usize::MAX;
        for city in cities.iter() {
            if !visited.contains(city) {
                let mut visited = visited.clone();
                visited.insert(*city);
                let this =
                    solve(&cities, &paths, city, visited) + paths.get(&(current, *city)).unwrap();
                if this < min {
                    min = this;
                }
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = [
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string(),
        ];

        assert_eq!(shortest(&lines), 605);
    }
}
