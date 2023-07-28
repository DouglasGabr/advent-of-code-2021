use std::ops::{Deref, DerefMut};

fn main() {
    let input = include_str!("../inputs/day4.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Clone, Copy)]
struct Cell(u64, bool);

struct Board([[Cell; 5]; 5]);
impl Board {
    fn is_winning(&self) -> bool {
        for row_idx in 0..5 {
            if self[row_idx].iter().all(|c| c.1) {
                return true;
            }
        }
        for col_idx in 0..5 {
            if self.iter().all(|row| row[col_idx].1) {
                return true;
            }
        }
        return false;
    }

    fn read_num(&mut self, num: u64) {
        for row in self.iter_mut() {
            for cell in row.iter_mut() {
                if cell.0 == num {
                    cell.1 = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u64 {
        self.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| !cell.1)
            .map(|cell| cell.0)
            .sum()
    }
}

impl Deref for Board {
    type Target = [[Cell; 5]; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn part1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let mut draws = sections
        .next()
        .expect("first row")
        .split(',')
        .map(|c| c.parse().expect("valid number"));
    let mut boards: Vec<Board> = sections
        .map(|board_str| {
            let mut board = Board([[Cell(0u64, false); 5]; 5]);
            for (row, line) in board_str.lines().enumerate() {
                for (col, c) in line.split_ascii_whitespace().enumerate() {
                    board[row][col] = Cell(c.parse().expect("valid number"), false);
                }
            }
            return board;
        })
        .collect();
    while let Some(num) = draws.next() {
        for board in boards.iter_mut() {
            board.read_num(num);
        }
        for board in boards.iter() {
            if board.is_winning() {
                return board.sum_unmarked() * num;
            }
        }
    }
    panic!("no winning board found")
}
fn part2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let mut draws = sections
        .next()
        .expect("first row")
        .split(',')
        .map(|c| c.parse().expect("valid number"));
    let mut boards: Vec<Board> = sections
        .map(|board_str| {
            let mut board = Board([[Cell(0u64, false); 5]; 5]);
            for (row, line) in board_str.lines().enumerate() {
                for (col, c) in line.split_ascii_whitespace().enumerate() {
                    board[row][col] = Cell(c.parse().expect("valid number"), false);
                }
            }
            return board;
        })
        .collect();
    while let Some(num) = draws.next() {
        for board in boards.iter_mut() {
            board.read_num(num);
        }
        if boards.len() == 1 && boards[0].is_winning() {
            return boards[0].sum_unmarked() * num;
        }
        boards = boards
            .into_iter()
            .filter(|board| !board.is_winning())
            .collect::<Vec<_>>();
    }
    panic!("no winning board found")
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), 1924);
    }
}
