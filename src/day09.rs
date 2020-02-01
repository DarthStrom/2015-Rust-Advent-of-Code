use crate::input::get_lines;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn run() {
    let lines = get_lines("day09");

    println!("part1: {}", shortest(&lines));

    println!("part2: {}", longest(&lines));
}

#[derive(Clone)]
enum Method {
    Min,
    Max,
}

impl Copy for Method {}

fn prep(lines: &[String]) -> (HashMap<(&str, &str), usize>, HashSet<&str>) {
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

    (paths, cities)
}

fn shortest(lines: &[String]) -> usize {
    let (paths, cities) = prep(lines);

    solve(&paths, &cities, "start", HashSet::new(), Method::Min)
}

fn longest(lines: &[String]) -> usize {
    let (paths, cities) = prep(lines);

    solve(&paths, &cities, "start", HashSet::new(), Method::Max)
}

fn solve(
    paths: &HashMap<(&str, &str), usize>,
    cities: &HashSet<&str>,
    current: &str,
    visited: HashSet<&str>,
    method: Method,
) -> usize {
    if visited.len() == cities.len() {
        0
    } else {
        let mut result = match method {
            Method::Min => usize::MAX,
            Method::Max => 0,
        };
        for city in cities.iter() {
            if !visited.contains(city) {
                let mut visited = visited.clone();
                visited.insert(*city);
                let this = solve(&paths, &cities, city, visited, method)
                    + paths.get(&(current, *city)).unwrap();
                if match method {
                    Method::Min => this < result,
                    Method::Max => this > result,
                } {
                    result = this;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_path() {
        let lines = [
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string(),
        ];

        assert_eq!(shortest(&lines), 605);
    }

    #[test]
    fn longest_path() {
        let lines = [
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string(),
        ];

        assert_eq!(longest(&lines), 982);
    }
}
