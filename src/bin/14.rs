use std::hash::Hasher;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hash,
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines().peekable();
    let num_cols = lines.peek().unwrap().len();
    let mut free_space = vec![0; num_cols];
    let mut settled_rocks = vec![];
    for (row, line) in lines.enumerate() {
        settled_rocks.push(0);
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => free_space[col] = row + 1,
                'O' => {
                    let settled = free_space[col];
                    settled_rocks[settled] += 1;
                    free_space[col] += 1;
                }
                _ => (),
            }
        }
    }
    let mut answer = 0;
    let total_rows = settled_rocks.len();
    for (row, rocks) in settled_rocks.into_iter().enumerate() {
        answer += (total_rows - row) * rocks;
    }
    Some(answer)
}

#[derive(Hash, PartialEq, Eq)]
struct Board {
    board: Vec<Vec<char>>,
}

fn score(b: &Board) -> usize {
    let mut answer = 0;
    let size = b.board.len();
    for row in 0..size {
        for col in 0..size {
            if b.board[row][col] == 'O' {
                answer += size - row;
            }
        }
    }
    answer
}
fn cycle(b: &mut Board) {
    let size = b.board.len();
    let mut free_space = vec![0; size];
    // North
    for row in 0..size {
        for col in 0..size {
            match b.board[row][col] {
                '#' => free_space[col] = row + 1,
                'O' => {
                    b.board[row][col] = '.';
                    b.board[free_space[col]][col] = 'O';
                    free_space[col] += 1;
                }
                _ => (),
            }
        }
    }
    // West
    free_space = vec![0; size];
    for col in 0..size {
        for row in 0..size {
            match b.board[row][col] {
                '#' => free_space[row] = col + 1,
                'O' => {
                    b.board[row][col] = '.';
                    b.board[row][free_space[row]] = 'O';
                    free_space[row] += 1;
                }
                _ => (),
            }
        }
    }
    // South
    free_space = vec![size - 1; size];
    for row in (0..size).rev() {
        for col in 0..size {
            match b.board[row][col] {
                '#' => free_space[col] = row.saturating_sub(1),
                'O' => {
                    b.board[row][col] = '.';
                    b.board[free_space[col]][col] = 'O';
                    free_space[col] = free_space[col].saturating_sub(1);
                }
                _ => (),
            }
        }
    }
    // East
    let mut free_space = vec![size - 1; size];
    for col in (0..size).rev() {
        for row in 0..size {
            match b.board[row][col] {
                '#' => free_space[row] = col.saturating_sub(1),
                'O' => {
                    b.board[row][col] = '.';
                    b.board[row][free_space[row]] = 'O';
                    free_space[row] = free_space[row].saturating_sub(1);
                }
                _ => (),
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut board = Board {
        board: input.lines().map(|l| l.chars().collect()).collect(),
    };
    let mut hashes = HashMap::new();
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hashes.insert(hasher.finish(), 0);

    let mut cycle_start = 0;
    let mut cycle_end = 0;
    for i in 1..1000 {
        cycle(&mut board);
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        if let Some(start) = hashes.insert(hasher.finish(), i) {
            println!("found cycle at {start} to {i}");
            cycle_start = start;
            cycle_end = i;
            break;
        }
    }
    assert_ne!(cycle_end, 0, "didn't find a cycle!");

    let cycle_len = cycle_end - cycle_start;
    let more_iterations_to_do = (1000000000 - cycle_start) % cycle_len;
    for _ in 0..more_iterations_to_do {
        cycle(&mut board);
    }
    Some(score(&board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
