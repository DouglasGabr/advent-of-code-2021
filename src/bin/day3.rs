use std::str::Lines;

fn main() {
    let input = include_str!("../inputs/day3.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut length = 0;
    let mut one_count: Vec<u64> = vec![];
    for line in input.lines() {
        length += 1;
        one_count.resize(line.len(), 0);
        for (idx, digit) in line.chars().enumerate() {
            match digit {
                '1' => {
                    one_count[idx] += 1;
                }
                '0' => {}
                _ => panic!("invalid digit"),
            }
        }
    }
    let gamma_rate = u64::from_str_radix(
        &one_count
            .iter()
            .map(|&ones| if ones > length / 2 { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .expect("binary number");
    let epsilon_rate = gamma_rate ^ !(u64::MAX << one_count.len());

    return gamma_rate * epsilon_rate;
}

fn part2(input: &str) -> u64 {
    let line_length = input.lines().next().unwrap().len();
    let lines_vec = input.lines().collect::<Vec<_>>();
    fn find_rating(lines: &Vec<&str>, idx: usize, line_count: usize, most_common: bool) -> u64 {
        let num_of_ones: u64 = lines
            .iter()
            .map(|line| u64::from_str_radix(line, 2).expect("binary number"))
            .map(|n| (n >> idx) & 1)
            .sum();
        let num_of_zeros = line_count as u64 - num_of_ones;
        let valid_lines = lines
            .iter()
            .filter(|line| {
                let n = u64::from_str_radix(line, 2).expect("binary number");
                (n >> idx) & 1
                    == if most_common {
                        (num_of_ones >= num_of_zeros) as u64
                    } else {
                        (num_of_ones < num_of_zeros) as u64
                    }
            })
            .map(|&line| line)
            .collect::<Vec<_>>();
        if valid_lines.len() == 1 {
            return u64::from_str_radix(valid_lines[0], 2).expect("binary number");
        } else {
            return find_rating(&valid_lines, idx - 1, valid_lines.len(), most_common);
        }
    }

    let oxygen_generator_rating = find_rating(&lines_vec, line_length - 1, lines_vec.len(), true);
    let co2_scrubber_rating = find_rating(&lines_vec, line_length - 1, lines_vec.len(), false);

    return oxygen_generator_rating * co2_scrubber_rating;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "00100\n\
                                      11110\n\
                                      10110\n\
                                      10111\n\
                                      10101\n\
                                      01111\n\
                                      00111\n\
                                      11100\n\
                                      10000\n\
                                      11001\n\
                                      00010\n\
                                      01010";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), 198);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), 230);
    }
}
