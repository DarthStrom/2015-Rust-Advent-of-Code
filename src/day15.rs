use crate::input::get_lines;
use std::collections::HashMap;

pub fn run() {
    let lines = get_lines("day15");

    println!("part1: {}", best_score(&lines, None));

    println!("part2: {}", best_score(&lines, Some(500)));
}

fn best_score(lines: &[String], calories: Option<u32>) -> i32 {
    let ingredients = lines
        .iter()
        .map(|line| {
            let line_without_commas = line.replace(",", "");
            let parts = line_without_commas.split(' ').collect::<Vec<_>>();
            let name = parts[0].replace(":", "");
            let capacity = parts[2].parse::<i32>().unwrap();
            let durability = parts[4].parse::<i32>().unwrap();
            let flavor = parts[6].parse::<i32>().unwrap();
            let texture = parts[8].parse::<i32>().unwrap();
            let calories = parts[10].parse::<u32>().unwrap();
            (
                name,
                Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
        })
        .collect::<HashMap<String, Ingredient>>();

    let mut all_cookies: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
    for frosting_amt in 0..=100 {
        for candy_amt in 0..100 - frosting_amt {
            for butterscotch_amt in 0..100 - candy_amt - frosting_amt {
                let sugar_amt = 100 - frosting_amt - candy_amt - butterscotch_amt;

                let frosting = &ingredients["Frosting"];
                let candy = &ingredients["Candy"];
                let butterscotch = &ingredients["Butterscotch"];
                let sugar = &ingredients["Sugar"];
                let mut capacity = frosting_amt * frosting.capacity
                    + candy_amt * candy.capacity
                    + butterscotch_amt * butterscotch.capacity
                    + sugar_amt * sugar.capacity;
                let mut durability = frosting_amt * frosting.durability
                    + candy_amt * candy.durability
                    + butterscotch_amt * butterscotch.durability
                    + sugar_amt * sugar.durability;
                let mut flavor = frosting_amt * frosting.flavor
                    + candy_amt * candy.flavor
                    + butterscotch_amt * butterscotch.flavor
                    + sugar_amt * sugar.flavor;
                let mut texture = frosting_amt * frosting.texture
                    + candy_amt * candy.texture
                    + butterscotch_amt * butterscotch.texture
                    + sugar_amt * sugar.texture;
                if capacity < 0 {
                    capacity = 0;
                }
                if durability < 0 {
                    durability = 0;
                }
                if flavor < 0 {
                    flavor = 0;
                }
                if texture < 0 {
                    texture = 0;
                }
                let score = capacity * durability * flavor * texture;

                match calories {
                    Some(c) => {
                        let total_cal = frosting_amt as u32 * frosting.calories
                            + candy_amt as u32 * candy.calories
                            + butterscotch_amt as u32 * butterscotch.calories
                            + sugar_amt as u32 * sugar.calories;
                        if total_cal == c {
                            all_cookies.insert(
                                (frosting_amt, candy_amt, butterscotch_amt, sugar_amt),
                                score,
                            );
                        }
                    }
                    None => {
                        all_cookies.insert(
                            (frosting_amt, candy_amt, butterscotch_amt, sugar_amt),
                            score,
                        );
                    }
                }
            }
        }
    }

    *all_cookies.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}
