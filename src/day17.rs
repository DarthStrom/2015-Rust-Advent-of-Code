use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day17");

    println!("part1: {}", combinations(&lines, 150));

    println!("part2: {}", ways_to_use_min_containers(&lines, 150));
}

fn combinations(lines: &[String], volume: i32) -> usize {
    let containers = get_containers(lines);

    find_combinations(containers.as_slice(), volume, containers.len() as i32, 0)
}

fn ways_to_use_min_containers(lines: &[String], volume: i32) -> usize {
    let containers = get_containers(lines);

    let mut i = 1;
    let mut result = 0;
    while result == 0 {
        result = find_combinations(containers.as_slice(), volume, i, 0);
        i += 1;
    }
    result
}

fn get_containers(lines: &[String]) -> Vec<i32> {
    let mut result = lines
        .iter()
        .map(String::as_str)
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();
    result.sort_unstable_by(|a, b| b.cmp(&a));
    result
}

fn find_combinations(containers: &[i32], volume: i32, remaining: i32, i: usize) -> usize {
    if remaining < 0 {
        0
    } else if volume == 0 {
        1
    } else if i == containers.len() || volume < 0 {
        0
    } else {
        find_combinations(containers, volume, remaining, i + 1)
            + find_combinations(containers, volume - containers[i], remaining - 1, i + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = [
            "15".to_string(),
            "10".to_string(),
            "5".to_string(),
            "20".to_string(),
            "5".to_string(),
        ];

        assert_eq!(combinations(&lines, 25), 4);
    }

    #[test]
    fn example2() {
        let lines = [
            "15".to_string(),
            "10".to_string(),
            "5".to_string(),
            "20".to_string(),
            "5".to_string(),
        ];

        assert_eq!(ways_to_use_min_containers(&lines, 25), 3);
    }
}
