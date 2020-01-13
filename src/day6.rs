use crate::input::get_lines;
use regex::Regex;
use std::fmt;

pub fn run() {
    let mut lights = Grid::new(1000, 1000);
    let lines = get_lines("day6");

    for line in lines {
        lights.do_instruction(&line);
    }

    println!("part2: {}", lights.count_brightness());
}

struct Grid {
    lights: Vec<Vec<usize>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.lights {
            for &light in row {
                write!(f, "{}", light)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(columns: usize, rows: usize) -> Self {
        let mut lights = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..columns {
                row.push(0);
            }
            lights.push(row);
        }

        Self { lights }
    }

    fn count_brightness(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum()
    }

    fn turn_on(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        self.increase(top_left, bottom_right, 1);
    }

    fn turn_off(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        self.increase(top_left, bottom_right, -1);
    }
    fn toggle(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        self.increase(top_left, bottom_right, 2);
    }

    fn increase(&mut self, top_left: (usize, usize), bottom_right: (usize, usize), by: i32) {
        for y in top_left.1..=bottom_right.1 {
            for x in top_left.0..=bottom_right.0 {
                let light = self.lights[y][x] as i32;
                if light + by < 0 {
                    self.lights[y][x] = 0;
                } else {
                    self.lights[y][x] = (light + by) as usize
                }
            }
        }
    }

    fn do_instruction(&mut self, instruction: &str) {
        let re = Regex::new(r"(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        if let Some(caps) = re.captures(instruction) {
            let func = &caps[1];
            let left = caps[2].parse::<usize>().unwrap();
            let top = caps[3].parse::<usize>().unwrap();
            let right = caps[4].parse::<usize>().unwrap();
            let bottom = caps[5].parse::<usize>().unwrap();

            match func {
                "turn on" => self.turn_on((left, top), (right, bottom)),
                "toggle" => self.toggle((left, top), (right, bottom)),
                "turn off" => self.turn_off((left, top), (right, bottom)),
                _ => panic!("unknown function"),
            }
        } else {
            panic!("didn't capture on instruction: '{}'", instruction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut grid = Grid::new(4, 4);
        assert_eq!(format!("{}", grid), "0000\n0000\n0000\n0000\n".to_string());

        grid.do_instruction("turn on 0,0 through 3,3");
        assert_eq!(format!("{}", grid), "1111\n1111\n1111\n1111\n".to_string());

        grid.do_instruction("toggle 0,0 through 3,0");
        assert_eq!(format!("{}", grid), "3333\n1111\n1111\n1111\n".to_string());

        grid.do_instruction("turn off 1,1 through 2,2");
        assert_eq!(format!("{}", grid), "3333\n1001\n1001\n1111\n".to_string());
    }
}
