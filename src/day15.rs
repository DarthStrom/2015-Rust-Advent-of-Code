use crate::input::get_lines;
use std::collections::HashMap;

pub fn run() {
    let lines = get_lines("day15");

    println!("part1: {:?}", best_score(&lines));
}

fn best_score(lines: &[String]) -> i32 {
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
    for frosting in 0..=100 {
        for candy in 0..100 - frosting {
            for butterscotch in 0..100 - candy - frosting {
                let sugar = 100 - frosting - candy - butterscotch;
                let mut capacity = frosting * ingredients["Frosting"].capacity
                    + candy * ingredients["Candy"].capacity
                    + butterscotch * ingredients["Butterscotch"].capacity
                    + sugar * ingredients["Sugar"].capacity;
                let mut durability = frosting * ingredients["Frosting"].durability
                    + candy * ingredients["Candy"].durability
                    + butterscotch * ingredients["Butterscotch"].durability
                    + sugar * ingredients["Sugar"].durability;
                let mut flavor = frosting * ingredients["Frosting"].flavor
                    + candy * ingredients["Candy"].flavor
                    + butterscotch * ingredients["Butterscotch"].flavor
                    + sugar * ingredients["Sugar"].flavor;
                let mut texture = frosting * ingredients["Frosting"].texture
                    + candy * ingredients["Candy"].texture
                    + butterscotch * ingredients["Butterscotch"].texture
                    + sugar * ingredients["Sugar"].texture;
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
                all_cookies.insert((frosting, candy, butterscotch, sugar), score);
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
