#[derive(PartialEq, Debug)]
enum CommandParseError {
    Format,
    InvalidCommand,
    InvalidUnit,
}

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl TryFrom<&str> for Command {
    type Error = CommandParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (command, unit) = value.split_once(' ').ok_or(CommandParseError::Format)?;
        let unit = unit
            .parse::<i64>()
            .map_err(|_| CommandParseError::InvalidUnit)?;
        return match command {
            "forward" => Ok(Self::Forward(unit)),
            "down" => Ok(Self::Down(unit)),
            "up" => Ok(Self::Up(unit)),
            _ => Err(CommandParseError::InvalidCommand),
        };
    }
}

struct Submarine {
    horizontal_position: i64,
    depth: i64,
    aim: i64,
}

impl Submarine {
    fn new() -> Self {
        Self {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::Forward(distance) => self.horizontal_position += distance,
            Command::Down(distance) => self.depth += distance,
            Command::Up(distance) => self.depth -= distance,
        }
    }

    fn execute_with_aim(&mut self, command: Command) {
        match command {
            Command::Forward(distance) => {
                self.horizontal_position += distance;
                self.depth += distance * self.aim;
            }
            Command::Down(distance) => self.aim += distance,
            Command::Up(distance) => self.aim -= distance,
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day2.txt");
    println!("Part 1: {}", part1(input).unwrap());
    println!("Part 2: {}", part2(input).unwrap());
}

fn part1(input: &str) -> Result<i64, CommandParseError> {
    let commands = input
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let mut submarine = Submarine::new();
    for command in commands {
        submarine.execute(command);
    }
    return Ok(submarine.horizontal_position * submarine.depth);
}
fn part2(input: &str) -> Result<i64, CommandParseError> {
    let commands = input
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let mut submarine = Submarine::new();
    for command in commands {
        submarine.execute_with_aim(command);
    }
    return Ok(submarine.horizontal_position * submarine.depth);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "forward 5\n\
                                      down 5\n\
                                      forward 8\n\
                                      up 3\n\
                                      down 8\n\
                                      forward 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), Ok(150));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), Ok(900));
    }
}
