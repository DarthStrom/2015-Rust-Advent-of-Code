fn sq_ft_wrapping_paper(size: &str) -> u32 {
    let dimensions = size
        .split('x')
        .take(3)
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();

    let sides = [
        dimensions[0] * dimensions[1],
        dimensions[1] * dimensions[2],
        dimensions[2] * dimensions[0],
    ];

    let smallest_side = sides.iter().min().unwrap();

    sides.iter().map(|s| s * 2).sum::<u32>() + smallest_side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(sq_ft_wrapping_paper("2x3x4"), 58);
    }

    #[test]
    fn example2() {
        assert_eq!(sq_ft_wrapping_paper("1x1x10"), 43);
    }
}
