use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day5");

    println!("part1: {}", lines.iter().filter(|l| nice(&l)).count());

    println!("part2: {}", lines.iter().filter(|l| nice2(&l)).count());
}

fn three_vowels(input: &str) -> bool {
    let mut vowels = 0;
    for c in input.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            vowels += 1;
            if vowels >= 3 {
                return true;
            }
        }
    }
    false
}

fn double_letter(input: &str) -> bool {
    let mut last_letter: Option<char> = None;
    for c in input.chars() {
        match last_letter {
            Some(last_letter) if last_letter == c => return true,
            _ => last_letter = Some(c),
        }
    }

    false
}

fn double_pair(input: &str) -> bool {
    for (i1, p1) in input.chars().collect::<Vec<_>>().windows(2).enumerate() {
        for p2 in input[i1 + 2..input.len()]
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
        {
            if p1[0] == p2[0] && p1[1] == p2[1] {
                return true;
            }
        }
    }
    false
}

fn sandwich(input: &str) -> bool {
    let mut two_back: Option<char> = None;
    let mut last_letter: Option<char> = None;

    for c in input.chars() {
        if two_back == Some(c) {
            return true;
        }
        two_back = last_letter;
        last_letter = Some(c);
    }
    false
}

fn nice(input: &str) -> bool {
    !input.contains("ab")
        && !input.contains("cd")
        && !input.contains("pq")
        && !input.contains("xy")
        && three_vowels(input)
        && double_letter(input)
}

fn nice2(input: &str) -> bool {
    double_pair(input) && sandwich(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn example2() {
        assert!(nice("aaa"));
    }

    #[test]
    fn example3() {
        assert!(!nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn example4() {
        assert!(!nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn example5() {
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn example6() {
        assert!(nice2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn example7() {
        assert!(nice2("xxyxx"));
    }

    #[test]
    fn example8() {
        assert!(!nice2("uurcxstgmygtbstg"));
    }

    #[test]
    fn example9() {
        assert!(!nice2("ieodomkazucvgmuy"));
    }
}
