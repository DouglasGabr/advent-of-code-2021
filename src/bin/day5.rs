use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day5.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

fn parse_coordinates(input: &str) -> Result<Vec<((u64, u64), (u64, u64))>> {
    input
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .ok_or(anyhow!("Invalid line"))
                .and_then(|(a, b)| {
                    Ok((
                        a.split_once(',').ok_or(anyhow!("Invalid coordinate"))?,
                        b.split_once(',').ok_or(anyhow!("Invalid coordinate"))?,
                    ))
                })
                .and_then(|(a, b)| {
                    Ok((
                        (a.0.parse::<u64>()?, a.1.parse::<u64>()?),
                        (b.0.parse::<u64>()?, b.1.parse::<u64>()?),
                    ))
                })
        })
        .collect()
}

fn part1(input: &str) -> Result<u64> {
    let coords: Vec<((u64, u64), (u64, u64))> = parse_coordinates(input)?;
    let mut map: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for vent in coords
        .iter()
        .filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
    {
        let x1 = vent.0 .0.min(vent.1 .0);
        let x2 = vent.0 .0.max(vent.1 .0);
        let y1 = vent.0 .1.min(vent.1 .1);
        let y2 = vent.0 .1.max(vent.1 .1);
        for x in x1..=x2 {
            for y in y1..=y2 {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    return Ok(map.into_iter().filter(|(_, v)| *v > 1).count() as u64);
}
fn part2(input: &str) -> Result<u64> {
    let coords: Vec<((u64, u64), (u64, u64))> = parse_coordinates(input)?;
    let mut map: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for vent in coords {
        let delta = (
            (vent.1 .0 as i64 - vent.0 .0 as i64).signum(),
            (vent.1 .1 as i64 - vent.0 .1 as i64).signum(),
        );
        let mut pos = vent.0;
        while pos != vent.1 {
            *map.entry(pos).or_insert(0) += 1;
            pos = (
                (pos.0 as i64 + delta.0) as u64,
                (pos.1 as i64 + delta.1) as u64,
            );
        }
        *map.entry(pos).or_insert(0) += 1;
    }
    return Ok(map.into_iter().filter(|(_, v)| *v > 1).count() as u64);
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_INPUT: &'static str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 5);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 12);
    }
}
