use std::char;

pub fn run() {
    println!(
        "part1: {}",
        look_and_say_times(40, "1113122113").chars().count()
    );

    println!(
        "part2: {}",
        look_and_say_times(50, "1113122113").chars().count()
    );
}

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut number: Option<char> = None;
    let mut count = 0;

    for c in s.chars() {
        match number {
            None => {
                count = 1;
                number = Some(c);
            }
            Some(same) if same == c => count += 1,
            Some(different) => {
                result.push(char::from_digit(count, 10).unwrap());
                result.push(different);
                count = 1;
                number = Some(c);
            }
        }
    }
    result.push(char::from_digit(count, 10).unwrap());
    result.push(number.unwrap());

    result
}

fn look_and_say_times(times: u32, s: &str) -> String {
    let mut result = s.to_string();

    for _ in 0..times {
        result = look_and_say(&result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(look_and_say_times(5, "1"), "312211");
    }
}
