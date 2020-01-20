use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day14");

    println!("part1: {}", distance_of_winning_reindeer(&lines, 2503));

    println!("part2: {}", points_of_winning_reindeer(&lines, 2503));
}

#[derive(Hash, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    time_going: u32,
    time_resting: u32,
    distance: u32,
    points: u32,
}

fn distance_of_winning_reindeer(lines: &[String], seconds: u32) -> u32 {
    let mut reindeers = populate_reindeers(lines);

    for time in 0..seconds {
        for reindeer in reindeers.iter_mut() {
            let cycle = reindeer.time_going + reindeer.time_resting;
            if time % cycle < reindeer.time_going {
                reindeer.distance += reindeer.speed;
            }
        }
    }

    reindeers
        .iter()
        .max_by(|a, b| a.distance.cmp(&b.distance))
        .unwrap()
        .distance
}

fn points_of_winning_reindeer(lines: &[String], seconds: u32) -> u32 {
    let mut reindeers = populate_reindeers(lines);

    for time in 0..seconds {
        for reindeer in reindeers.iter_mut() {
            let cycle = reindeer.time_going + reindeer.time_resting;
            if time % cycle < reindeer.time_going {
                reindeer.distance += reindeer.speed;
            }
        }
        let max_distance = reindeers
            .iter()
            .max_by(|a, b| a.distance.cmp(&b.distance))
            .unwrap()
            .distance;
        for reindeer in reindeers.iter_mut().filter(|r| r.distance == max_distance) {
            reindeer.points += 1;
        }
    }

    reindeers
        .iter()
        .max_by(|a, b| a.points.cmp(&b.points))
        .unwrap()
        .points
}

fn populate_reindeers(lines: &[String]) -> Vec<Reindeer> {
    let mut reindeers: Vec<Reindeer> = vec![];

    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();

        reindeers.push(Reindeer {
            name: parts[0].to_string(),
            speed: parts[3].parse::<u32>().unwrap(),
            time_going: parts[6].parse::<u32>().unwrap(),
            time_resting: parts[13].parse::<u32>().unwrap(),
            distance: 0,
            points: 0,
        });
    }

    reindeers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = [
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_string(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
                .to_string(),
        ];

        assert_eq!(distance_of_winning_reindeer(&lines, 1000), 1120);
    }

    #[test]
    fn example2() {
        let lines = [
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_string(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
                .to_string(),
        ];

        assert_eq!(points_of_winning_reindeer(&lines, 1000), 689);
    }
}
