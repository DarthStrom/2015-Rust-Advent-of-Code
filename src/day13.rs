use crate::input::get_lines;
use permutohedron::heap_recursive;
use std::collections::HashMap;

pub fn run() {
    let lines = get_lines("day13");

    println!("part1: {}", optimal_happiness(&lines));

    println!("part2: {}", optimal_happiness_plus_self(&lines));
}

fn optimal_happiness(lines: &[String]) -> i32 {
    let mut table: Vec<String> = vec![];
    let mut map: HashMap<(String, String), i32> = HashMap::new();
    for line in lines {
        let words = line.split(' ').collect::<Vec<_>>();
        let subject = words[0].to_string();
        let outcome = words[2];
        let amount = words[3].parse::<i32>().unwrap()
            * match outcome {
                "gain" => 1,
                "lose" => -1,
                unknown => panic!("They would what? {}?", unknown),
            };
        let other = words[10].replace(".", "");
        if !table.contains(&subject) {
            table.push(subject.clone());
        }
        map.insert((subject, other), amount);
    }

    let mut result = 0;
    heap_recursive(&mut table, |permutation| {
        let this = happiness(permutation, &map);
        if this > result {
            result = this;
        }
    });

    result
}

fn optimal_happiness_plus_self(lines: &[String]) -> i32 {
    let me = "Self".to_string();
    let mut table: Vec<String> = vec![me.clone()];
    let mut map: HashMap<(String, String), i32> = HashMap::new();
    for line in lines {
        let words = line.split(' ').collect::<Vec<_>>();
        let subject = words[0].to_string();
        let outcome = words[2];
        let amount = words[3].parse::<i32>().unwrap()
            * match outcome {
                "gain" => 1,
                "lose" => -1,
                unknown => panic!("They would what? {}?", unknown),
            };
        let other = words[10].replace(".", "");
        if !table.contains(&subject) {
            table.push(subject.clone());
            map.insert((me.clone(), subject.clone()), 0);
        }
        map.insert((subject.clone(), other), amount);
        map.insert((subject, me.clone()), 0);
    }

    let mut result = 0;
    heap_recursive(&mut table, |permutation| {
        let this = happiness(permutation, &map);
        if this > result {
            result = this;
        }
    });

    result
}

fn happiness(table: &[String], map: &HashMap<(String, String), i32>) -> i32 {
    let inner = table
        .windows(2)
        .map(|window| {
            map[&(window[0].clone(), window[1].clone())]
                + map[&(window[1].clone(), window[0].clone())]
        })
        .sum::<i32>();
    inner
        + map[&(table[0].clone(), table[table.len() - 1].clone())]
        + map[&(table[table.len() - 1].clone(), table[0].clone())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = [
            "Alice would gain 54 happiness units by sitting next to Bob.".to_string(),
            "Alice would lose 79 happiness units by sitting next to Carol.".to_string(),
            "Alice would lose 2 happiness units by sitting next to David.".to_string(),
            "Bob would gain 83 happiness units by sitting next to Alice.".to_string(),
            "Bob would lose 7 happiness units by sitting next to Carol.".to_string(),
            "Bob would lose 63 happiness units by sitting next to David.".to_string(),
            "Carol would lose 62 happiness units by sitting next to Alice.".to_string(),
            "Carol would gain 60 happiness units by sitting next to Bob.".to_string(),
            "Carol would gain 55 happiness units by sitting next to David.".to_string(),
            "David would gain 46 happiness units by sitting next to Alice.".to_string(),
            "David would lose 7 happiness units by sitting next to Bob.".to_string(),
            "David would gain 41 happiness units by sitting next to Carol.".to_string(),
        ];

        assert_eq!(optimal_happiness(&lines), 330)
    }
}
