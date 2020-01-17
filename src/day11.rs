use std::collections::HashSet;

pub fn run() {
    let first_new_password = next("hepxcrrq");

    println!("part1: {}", first_new_password);
    println!("part2: {}", next(&first_new_password));
}

fn valid(password: &str) -> bool {
    includes_straight(password) && no_ambiguous_letters(password) && two_pairs(password)
}

fn includes_straight(password: &str) -> bool {
    let mut straight: Vec<char> = vec![];

    for c in password.chars() {
        match straight.len() {
            0 => straight.push(c),
            1 => {
                let last = straight[0] as u8;

                if c as u8 == last + 1 {
                    straight.push(c);
                } else {
                    straight = vec![c];
                }
            }
            2 => {
                let last = straight[1] as u8;
                if c as u8 == last + 1 {
                    return true;
                } else {
                    straight = vec![c];
                }
            }
            _ => panic!("too many things in straight"),
        }
    }

    false
}

fn no_ambiguous_letters(password: &str) -> bool {
    !password.contains('i') && !password.contains('o') && !password.contains('l')
}

fn two_pairs(password: &str) -> bool {
    let chars = password.chars().collect::<Vec<_>>();
    let mut pairs: HashSet<_> = HashSet::new();

    for window in chars.windows(2) {
        if window[0] == window[1] {
            pairs.insert(window);
        }
    }

    pairs.len() >= 2
}

fn next(password: &str) -> String {
    let mut current = increment(&password);

    while !valid(&current) {
        current = increment(&current);
    }

    current
}

fn increment(password: &str) -> String {
    let mut chars = password.chars().collect::<Vec<_>>();
    chars.reverse();
    let mut keep_going = increase(&mut chars[0]);

    let mut i = 1;
    while keep_going {
        keep_going = increase(&mut chars[i]);
        i += 1;
    }

    chars.reverse();
    chars.into_iter().collect()
}

fn increase(c: &mut char) -> bool {
    match c {
        'z' => {
            *c = 'a';
            true
        }
        _ => {
            let code = *c as u8 + 1;
            *c = code as char;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(next("abcdefgh"), "abcdffaa");
    }
}
