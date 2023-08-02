use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day6.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u64> {
    let mut fishes = input
        .split(",")
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    for _ in 0..80 {
        let mut fishes_to_add = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                fishes_to_add += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..fishes_to_add {
            fishes.push(8);
        }
    }

    return Ok(fishes.len() as u64);
}
fn part2(input: &str) -> Result<u64> {
    let mut fishes = [0u64; 9];
    for fish_lifespan in input.split(",").map(|x| x.parse::<u64>()) {
        fishes[fish_lifespan? as usize] += 1;
    }
    for _ in 0..256 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    return Ok(fishes.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 5934);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 26984457539);
    }
}
