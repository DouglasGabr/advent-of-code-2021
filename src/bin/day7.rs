use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day7.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    fn calculate_fuel_consumption(crab_positions: &[u32], target: u32) -> u32 {
        crab_positions.iter().map(|pos| pos.abs_diff(target)).sum()
    }

    let crab_positions = input
        .split(',')
        .map(|pos| pos.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    let smaller_position = *crab_positions.iter().min().ok_or(anyhow!("no crabs"))?;
    let largest_position = *crab_positions.iter().max().ok_or(anyhow!("no crabs"))?;
    let smaller_fuel_consumption = (smaller_position..=largest_position)
        .map(|target| calculate_fuel_consumption(&crab_positions, target))
        .min()
        .ok_or(anyhow!("no min fuel consumption"))?;
    Ok(smaller_fuel_consumption)
}

fn part2(input: &str) -> Result<u32> {
    fn calculate_fuel_consumption(crab_positions: &[u32], target: u32) -> u32 {
        crab_positions
            .iter()
            .map(|pos| pos.abs_diff(target))
            .map(|distance| (distance * (distance + 1)) / 2) // https://en.wikipedia.org/wiki/Triangular_number
            .sum()
    }

    let crab_positions = input
        .split(',')
        .map(|pos| pos.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    let smaller_position = *crab_positions.iter().min().ok_or(anyhow!("no crabs"))?;
    let largest_position = *crab_positions.iter().max().ok_or(anyhow!("no crabs"))?;
    let smaller_fuel_consumption = (smaller_position..=largest_position)
        .map(|target| calculate_fuel_consumption(&crab_positions, target))
        .min()
        .ok_or(anyhow!("no min fuel consumption"))?;
    Ok(smaller_fuel_consumption)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 37)
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 168)
    }
}
