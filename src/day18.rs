use crate::input::get_lines;
use std::fmt;

pub fn run() {
    let lines = get_lines("day18");
    let mut grid = Grid::new(&lines);

    for _ in 0..100 {
        grid.step();
    }

    println!("part1: {}", grid.lights_on());

    let mut stuck_grid = Grid::new_stuck(&lines);

    for _ in 0..100 {
        stuck_grid.step_stuck();
    }

    println!("part2: {}", stuck_grid.lights_on());
}

struct Grid {
    lights: Vec<Vec<bool>>,
}

impl Grid {
    fn new(lines: &[String]) -> Self {
        let lights = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        c => panic!("unknown symbol: {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { lights }
    }

    fn new_stuck(lines: &[String]) -> Self {
        let mut result = Self::new(lines);
        let last_column = result.lights[0].len() - 1;
        let last_row = result.lights.len() - 1;

        result.lights[0][0] = true;
        result.lights[0][last_column] = true;
        result.lights[last_row][0] = true;
        result.lights[last_row][last_column] = true;

        result
    }

    fn step(&mut self) {
        let initial_grid = self.lights.clone();

        for (j, row) in self.lights.iter_mut().enumerate() {
            for (i, light) in row.iter_mut().enumerate() {
                let neighbors = count_neighbors(&initial_grid, i, j);
                if *light {
                    match neighbors {
                        2 | 3 => (),
                        _ => *light = false,
                    }
                } else if let 3 = neighbors {
                    *light = true
                }
            }
        }
    }

    fn step_stuck(&mut self) {
        let initial_grid = self.lights.clone();
        let last_column = initial_grid[0].len() - 1;
        let last_row = initial_grid.len() - 1;

        for (j, row) in self.lights.iter_mut().enumerate() {
            for (i, light) in row.iter_mut().enumerate() {
                let neighbors = count_neighbors(&initial_grid, i, j);
                if ![
                    (0, 0),
                    (0, last_row),
                    (last_column, 0),
                    (last_column, last_row),
                ]
                .contains(&(i, j))
                {
                    if *light {
                        match neighbors {
                            2 | 3 => (),
                            _ => *light = false,
                        }
                    } else if let 3 = neighbors {
                        *light = true
                    }
                }
            }
        }
    }

    fn lights_on(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&light| *light).count())
            .sum()
    }
}

fn count_neighbors(grid: &[Vec<bool>], column: usize, row: usize) -> usize {
    let mut result = 0;

    let start_x = if column > 0 { column - 1 } else { 0 };
    let start_y = if row > 0 { row - 1 } else { 0 };
    let end_x = if column < grid[0].len() - 1 {
        column + 1
    } else {
        grid[0].len() - 1
    };
    let end_y = if row < grid.len() - 1 {
        row + 1
    } else {
        grid.len() - 1
    };

    for x in start_x..=end_x {
        for (y, grid_row) in grid.iter().enumerate().take(end_y + 1).skip(start_y) {
            if !(x == column && y == row) && grid_row[x] {
                result += 1
            }
        }
    }

    result
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .lights
            .iter()
            .map(|row| {
                row.iter()
                    .map(|state| match state {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = [
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ];

        let mut grid = Grid::new(&lines);

        assert_eq!(format!("{}", grid), lines.join("\n"));

        grid.step();

        assert_eq!(
            format!("{}", grid),
            "..##..\n..##.#\n...##.\n......\n#.....\n#.##.."
        );

        grid.step();

        assert_eq!(
            format!("{}", grid),
            "..###.\n......\n..###.\n......\n.#....\n.#...."
        );

        grid.step();

        assert_eq!(
            format!("{}", grid),
            "...#..\n......\n...#..\n..##..\n......\n......"
        );

        grid.step();

        assert_eq!(
            format!("{}", grid),
            "......\n......\n..##..\n..##..\n......\n......"
        );

        assert_eq!(grid.lights_on(), 4);
    }

    #[test]
    fn example2() {
        let lines = [
            "##.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####.#".to_string(),
        ];

        let mut grid = Grid::new_stuck(&lines);

        assert_eq!(format!("{}", grid), lines.join("\n"));

        grid.step_stuck();

        assert_eq!(
            format!("{}", grid),
            "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####"
        );

        grid.step_stuck();
        grid.step_stuck();
        grid.step_stuck();
        grid.step_stuck();

        assert_eq!(
            format!("{}", grid),
            "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#"
        );

        assert_eq!(grid.lights_on(), 17);
    }
}
