use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("part1: {}", day1::get_floor(&contents));
}

mod day1 {

    pub fn get_floor(directions: &str) -> i32 {
        let ups = directions.matches("(").count() as i32;
        let downs = directions.matches(")").count() as i32;
        ups - downs
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
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
    }
}
