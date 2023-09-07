use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day8.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let (_, output) = line.split_once(" | ").ok_or(anyhow!("Invalid input"))?;
        sum += output
            .split_ascii_whitespace()
            .filter(|x| matches!(x.len(), 3 | 4 | 2 | 7))
            .count() as u32;
    }
    return Ok(sum);
}

fn get_mask_from_entry(entry: &str) -> u8 {
    entry
        .chars()
        .map(|c| 1u8 << (c as u8) - ('a' as u8))
        .fold(0, |a, b| a | b)
}

fn part2(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut digits: [u8; 10] = [0; 10];
        let (input, output) = line.split_once(" | ").ok_or(anyhow!("Invalid input"))?;
        let (input, output) = (
            input.split_ascii_whitespace().collect::<Vec<&str>>(),
            output.split_ascii_whitespace().collect::<Vec<&str>>(),
        );
        while digits.iter().any(|&d| d == 0) {
            for &entry in input.iter() {
                let mask = get_mask_from_entry(entry);
                match entry.len() {
                    6 => {
                        if digits[4] > 0 {
                            let result = mask & digits[4];
                            if result == digits[4] {
                                digits[9] = mask;
                                continue;
                            }
                        }
                        if digits[1] > 0 && digits[8] > 0 {
                            let relevant = digits[1] ^ digits[8];
                            let result = mask & relevant;
                            if result == relevant {
                                digits[6] = mask;
                                continue;
                            }
                        }
                        if digits[6] > 0 && digits[9] > 0 && mask != digits[6] && mask != digits[9]
                        {
                            digits[0] = mask;
                        }
                    }
                    2 => {
                        digits[1] = mask;
                    }
                    5 => {
                        if digits[1] > 0 {
                            let result = mask & digits[1];
                            if result == digits[1] {
                                digits[3] = mask;
                                continue;
                            }
                            if digits[4] > 0 {
                                let relevant = digits[1] ^ digits[4];
                                let result = mask & relevant;
                                if result == relevant {
                                    digits[5] = mask;
                                    continue;
                                }
                            }
                        }
                        if digits[3] > 0 && digits[5] > 0 && mask != digits[3] && mask != digits[5]
                        {
                            digits[2] = mask;
                        }
                    }
                    4 => {
                        digits[4] = mask;
                    }
                    3 => {
                        digits[7] = mask;
                    }
                    7 => {
                        digits[8] = mask;
                    }
                    _ => unreachable!("Invalid entry length"),
                }
            }
        }
        for (exp, entry) in output.iter().rev().enumerate() {
            let mask = get_mask_from_entry(entry);
            let num = digits.iter().position(|&d| d == mask).unwrap() as u32;
            sum += num * 10u32.pow(exp as u32)
        }
    }
    return Ok(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 61229);
    }
}
