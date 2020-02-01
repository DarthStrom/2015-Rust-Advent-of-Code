use crate::input::get_lines;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashSet;

pub fn run() {
    let mut lines = get_lines("day19");
    let mut replacements: Vec<(String, String)> = vec![];
    let molecule = lines.pop().unwrap();

    for line in lines {
        if line == "" {
            break;
        } else {
            let parts = line.split(' ').collect::<Vec<_>>();
            replacements.push((parts[0].to_string(), parts[2].to_string()));
        }
    }

    println!(
        "part1: {:?}",
        distinct_molecules(&replacements.as_slice(), &molecule)
    );

    println!(
        "part2: {:?}",
        fewest_steps(&replacements.as_slice(), "e", &molecule)
    );
}

fn distinct_molecules(replacements: &[(String, String)], molecule: &str) -> usize {
    let mut molecules: HashSet<String> = HashSet::new();

    for (from, to) in replacements.iter() {
        let find = Regex::new(from).unwrap();

        for mat in find.find_iter(molecule) {
            let i = mat.start();
            let mut new = molecule.chars().take(i).collect::<Vec<_>>();
            new.append(&mut to.chars().collect::<Vec<_>>());
            new.append(&mut molecule.chars().skip(i + from.len()).collect::<Vec<_>>());
            molecules.insert(new.iter().collect::<String>());
        }
    }

    molecules.len()
}

fn fewest_steps(replacements: &[(String, String)], origin: &str, target: &str) -> usize {
    let mut result = 0;
    let mut reps = replacements.iter().collect::<Vec<_>>();

    let mut t = target.to_string();
    let mut iters = 0;
    while t != origin && iters < 10000 {
        let mut rng = thread_rng();
        reps.shuffle(&mut rng);
        let rep = reps[0];
        let from = rep.0.clone();
        let to = rep.1.clone();
        if t.contains(&to) {
            t = t.replacen(&to, &from, 1);
            println!("{:?}", t);
            result += 1;
        }
        iters += 1;
    }

    println!("{:?}", reps);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let replacements = [
            ("H".to_string(), "HO".to_string()),
            ("H".to_string(), "OH".to_string()),
            ("O".to_string(), "HH".to_string()),
        ];

        assert_eq!(distinct_molecules(&replacements, "HOH"), 4);
        assert_eq!(distinct_molecules(&replacements, "HOHOHO"), 7);
    }

    #[test]
    fn example2() {
        let replacements = [
            ("e".to_string(), "H".to_string()),
            ("e".to_string(), "O".to_string()),
            ("H".to_string(), "HO".to_string()),
            ("H".to_string(), "OH".to_string()),
            ("O".to_string(), "HH".to_string()),
        ];

        assert_eq!(fewest_steps(&replacements, "e", "HOH"), 3);
        assert_eq!(fewest_steps(&replacements, "e", "HOHOHO"), 6);
    }
}
