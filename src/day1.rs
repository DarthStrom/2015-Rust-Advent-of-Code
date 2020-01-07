pub fn get_floor(directions: &str) -> i32 {
    let ups = directions.matches("(").count() as i32;
    let downs = directions.matches(")").count() as i32;
    ups - downs
}

pub fn when_basement(directions: &str) -> usize {
    let mut location = 0;
    let mut position = 1;

    for (i, c) in directions.chars().enumerate() {
        match c {
            ')' => location -= 1,
            '(' => location += 1,
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
