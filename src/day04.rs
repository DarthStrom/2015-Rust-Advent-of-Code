const EASY: &str = "00000";
const HARD: &str = "000000";

pub fn run() {
    let input = "yzbqklnj";

    println!("part1: {}", get_easy_coin(input));

    println!("part2: {}", get_hard_coin(input));
}

fn get_easy_coin(input: &str) -> u32 {
    get_coin(input, EASY)
}

fn get_hard_coin(input: &str) -> u32 {
    get_coin(input, HARD)
}

fn get_coin(input: &str, prefix: &str) -> u32 {
    for x in 0.. {
        let secret = format!("{}{}", input, x);
        let digest = format!("{:x}", md5::compute(secret.as_bytes()));
        if digest.starts_with(prefix) {
            return x;
        }
    }
    panic!("finished with infinity already?");
}

#[cfg(test)]
mod tests {
    // commented because slowness

    // #[test]
    // fn example1() {
    //     assert_eq!(get_easy_coin("abcdef"), 609043);
    // }

    // #[test]
    // fn example2() {
    //     assert_eq!(get_easy_coin("pqrstuv"), 1048970);
    // }
}
