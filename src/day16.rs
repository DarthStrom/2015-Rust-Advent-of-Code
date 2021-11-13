use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day16");

    println!("part1: {:?}", sue_number(&lines));
}

#[derive(Debug, Default)]
struct Sue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

fn sue_number(lines: &[String]) -> u32 {
    let sues = lines
        .iter()
        .map(|line| {
            let line_trimmed = line.replace(':', "").replace(',', "");
            let parts = line_trimmed.split(' ').collect::<Vec<_>>();
            let number = parts[1].parse::<u32>().unwrap();
            let thing1 = parts[2];
            let thing1_count = parts[3].parse::<u32>().unwrap();
            let thing2 = parts[4];
            let thing2_count = parts[5].parse::<u32>().unwrap();
            let thing3 = parts[6];
            let thing3_count = parts[7].parse::<u32>().unwrap();
            let mut sue = Sue {
                number,
                ..Sue::default()
            };
            add_thing(&mut sue, thing1, thing1_count);
            add_thing(&mut sue, thing2, thing2_count);
            add_thing(&mut sue, thing3, thing3_count);
            sue
        })
        .filter(|sue| {
            (sue.children.is_none() || sue.children.unwrap_or(0) == 3)
                && (sue.cats.is_none() || sue.cats.unwrap_or(7) > 7)
                && (sue.samoyeds.is_none() || sue.samoyeds.unwrap_or(0) == 2)
                && (sue.pomeranians.is_none() || sue.pomeranians.unwrap_or(3) < 3)
                && (sue.akitas.is_none() || sue.akitas.unwrap_or(1) == 0)
                && (sue.vizslas.is_none() || sue.vizslas.unwrap_or(1) == 0)
                && (sue.goldfish.is_none() || sue.goldfish.unwrap_or(5) < 5)
                && (sue.trees.is_none() || sue.trees.unwrap_or(3) > 3)
                && (sue.cars.is_none() || sue.cars.unwrap_or(0) == 2)
                && (sue.perfumes.is_none() || sue.perfumes.unwrap_or(0) == 1)
        })
        .collect::<Vec<_>>();
    println!("{:?}", sues);
    sues.first().unwrap().number
}

fn add_thing(sue: &mut Sue, thing: &str, count: u32) {
    match thing {
        "children" => sue.children = Some(count),
        "cats" => sue.cats = Some(count),
        "samoyeds" => sue.samoyeds = Some(count),
        "pomeranians" => sue.pomeranians = Some(count),
        "akitas" => sue.akitas = Some(count),
        "vizslas" => sue.vizslas = Some(count),
        "goldfish" => sue.goldfish = Some(count),
        "trees" => sue.trees = Some(count),
        "cars" => sue.cars = Some(count),
        "perfumes" => sue.perfumes = Some(count),
        _ => (),
    }
}
