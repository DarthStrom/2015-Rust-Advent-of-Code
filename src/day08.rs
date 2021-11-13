use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day08");

    println!("part1: {}", overhead_chars(&lines));

    println!("part2: {}", encoding_chars(&lines));
}

fn encoded_chars(s: &str) -> usize {
    let mut result = 0;

    for c in s.chars() {
        if "\\\"".contains(c) {
            result += 1;
        }
        result += 1;
    }
    result += 2;

    result
}

fn literal_chars(s: &str) -> usize {
    s.chars().count()
}

fn string_chars(s: &str) -> usize {
    let mut result = 0;

    let mut escaped = false;
    let mut hexed = 0;
    for c in s.chars() {
        if escaped {
            if !"\\\"x".contains(c) {
                result += 1;
            }
            escaped = false;
            if c == 'x' {
                hexed = 1;
            } else {
                result += 1;
            }
        } else if hexed > 0 {
            let hex_digits = "0123456789abcdef";
            match hexed {
                1 => {
                    if !hex_digits.contains(c) {
                        result += 2;
                        hexed = 0;
                    } else {
                        hexed = 2;
                    }
                }
                2 => {
                    if !hex_digits.contains(c) {
                        result += 3;
                    }
                    hexed = 0;
                    result += 1;
                }
                _ => panic!("to many hexeds"),
            }
        } else {
            match c {
                '"' => (),
                '\\' => escaped = true,
                _ => result += 1,
            }
        }
    }

    result
}

fn encoding_chars(strings: &[String]) -> usize {
    strings
        .iter()
        .map(|s| encoded_chars(s) - literal_chars(s))
        .sum()
}

fn overhead_chars(strings: &[String]) -> usize {
    strings
        .iter()
        .map(|s| literal_chars(s) - string_chars(s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = [
            r#""""#.to_string(),
            r#""abc""#.to_string(),
            r#""aaa\"aaa""#.to_string(),
            r#""\x27""#.to_string(),
        ];
        assert_eq!(overhead_chars(&lines), 12);
    }

    #[test]
    fn example2() {
        let lines = [
            r#""""#.to_string(),
            r#""abc""#.to_string(),
            r#""aaa\"aaa""#.to_string(),
            r#""\x27""#.to_string(),
        ];
        assert_eq!(encoding_chars(&lines), 19);
    }
}
