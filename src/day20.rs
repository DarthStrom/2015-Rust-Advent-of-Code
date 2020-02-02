const ELVES: usize = 3_310_000;

pub fn run() {
    println!("part1: {}", lowest_house_to_get(33_100_000));
    println!("part2: {}", part_2());
}

fn lowest_house_to_get(input: usize) -> usize {
    let elves = input / 10;
    let mut houses: Vec<usize> = vec![elves];
    let mut house_number = elves;

    for elf in 1..elves {
        for house in (elf..elves).step_by(elf) {
            if houses.get(house).is_some() {
                houses[house] += elf;
            } else {
                houses.push(elf);
            }
            let presents = houses[house];
            if presents >= elves && house < house_number {
                house_number = house;
            }
        }
    }

    house_number
}

fn part_2() -> usize {
    let mut houses: Vec<usize> = vec![0; ELVES];
    let mut house_number = ELVES;

    for elf in 1..ELVES {
        for (visits, house) in (elf..ELVES).step_by(elf).enumerate() {
            houses[house] += elf * 11;
            let presents = houses[house];
            if presents >= ELVES * 10 && elf < house_number {
                house_number = house;
            }

            if visits >= 50 {
                break;
            }
        }
    }

    house_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(lowest_house_to_get(149), 8);
    }
}
