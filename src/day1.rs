use std::fs;

const UP: char = '(';
const DOWN: char = ')';

pub fn run() {
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("part1: {}", get_floor(&contents));

    println!("part2: {}", when_basement(&contents));
}

fn get_floor(directions: &str) -> i32 {
    let ups = directions.matches(UP).count() as i32;
    let downs = directions.matches(DOWN).count() as i32;
    ups - downs
}

fn when_basement(directions: &str) -> usize {
    let mut location = 0;
    let mut position = 1;

    for (i, c) in directions.chars().enumerate() {
        match c {
            DOWN => location -= 1,
            UP => location += 1,
            _ => (),
        }
        if location == -1 {
            position = i + 1;
            break;
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(get_floor("(())"), 0);
        assert_eq!(get_floor("()()"), 0);
        assert_eq!(get_floor("((("), 3);
        assert_eq!(get_floor("(()(()("), 3);
        assert_eq!(get_floor("))((((("), 3);
        assert_eq!(get_floor("())"), -1);
        assert_eq!(get_floor("))("), -1);
        assert_eq!(get_floor(")))"), -3);
        assert_eq!(get_floor(")())())"), -3);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(when_basement(")"), 1);
        assert_eq!(when_basement("()())"), 5);
    }
}
