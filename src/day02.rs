use crate::input::get_lines;

pub fn run() {
    let lines = get_lines("day02");

    println!(
        "part1: {}",
        lines.iter().map(|l| sq_ft_wrapping_paper(l)).sum::<u32>()
    );

    println!("part2: {}", lines.iter().map(|l| ft_ribbon(l)).sum::<u32>());
}

fn ft_ribbon(size: &str) -> u32 {
    let dimensions = get_dimensions(size);

    let perimeters = [
        dimensions[0] * 2 + dimensions[1] * 2,
        dimensions[1] * 2 + dimensions[2] * 2,
        dimensions[2] * 2 + dimensions[0] * 2,
    ];

    let smallest_perimeter = perimeters.iter().min().unwrap();

    let volume = dimensions[0] * dimensions[1] * dimensions[2];

    smallest_perimeter + volume
}

fn sq_ft_wrapping_paper(size: &str) -> u32 {
    let dimensions = get_dimensions(size);

    let sides = [
        dimensions[0] * dimensions[1],
        dimensions[1] * dimensions[2],
        dimensions[2] * dimensions[0],
    ];

    let smallest_side = sides.iter().min().unwrap();

    sides.iter().map(|s| s * 2).sum::<u32>() + smallest_side
}

fn get_dimensions(size: &str) -> Vec<u32> {
    size.split('x')
        .take(3)
        .flat_map(str::parse::<u32>)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paper_example1() {
        assert_eq!(sq_ft_wrapping_paper("2x3x4"), 58);
    }

    #[test]
    fn paper_example2() {
        assert_eq!(sq_ft_wrapping_paper("1x1x10"), 43);
    }

    #[test]
    fn ribbon_example1() {
        assert_eq!(ft_ribbon("2x3x4"), 34);
    }

    #[test]
    fn ribbon_example2() {
        assert_eq!(ft_ribbon("1x1x10"), 14);
    }
}
