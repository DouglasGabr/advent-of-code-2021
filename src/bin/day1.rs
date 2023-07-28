fn main() {
    let input = include_str!("../inputs/day1.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("each line to be a number"))
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|depths| depths[1] > depths[0])
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("each line to be a number"))
        .collect::<Vec<_>>()
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|depths| depths[1] > depths[0])
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "199\n\
                                      200\n\
                                      208\n\
                                      210\n\
                                      200\n\
                                      207\n\
                                      240\n\
                                      269\n\
                                      260\n\
                                      263";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), 5);
    }
}
