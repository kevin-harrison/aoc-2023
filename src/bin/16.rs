advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn to_idx(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        }
    }
}

fn solve(board: &Vec<Vec<char>>, start: ((usize, usize), Direction)) -> usize {
    let mut visited = vec![vec![[false, false, false, false]; board[0].len()]; board.len()];
    let mut lights = vec![start];
    while let Some((mut coords, mut dir)) = lights.pop() {
        loop {
            // println!("{coords:?}   {dir:?}   {}", board[coords.0][coords.1]);
            if visited[coords.0][coords.1][dir.to_idx()] {
                // println!("");
                break;
            } else {
                visited[coords.0][coords.1][dir.to_idx()] = true;
            }
            dir = match (board[coords.0][coords.1], dir) {
                ('|', d) if d == Direction::Left || d == Direction::Right => {
                    lights.push((coords, Direction::Up));
                    Direction::Down
                }
                ('-', d) if d == Direction::Up || d == Direction::Down => {
                    lights.push((coords, Direction::Left));
                    Direction::Right
                }
                ('\\', Direction::Up) => Direction::Left,
                ('\\', Direction::Left) => Direction::Up,
                ('\\', Direction::Down) => Direction::Right,
                ('\\', Direction::Right) => Direction::Down,
                ('/', Direction::Up) => Direction::Right,
                ('/', Direction::Left) => Direction::Down,
                ('/', Direction::Down) => Direction::Left,
                ('/', Direction::Right) => Direction::Up,
                (_, d) => d,
            };
            match dir {
                Direction::Up => coords.0 = coords.0.saturating_sub(1),
                Direction::Left => coords.1 = coords.1.saturating_sub(1),
                Direction::Down => coords.0 = (coords.0 + 1).min(board.len() - 1),
                Direction::Right => coords.1 = (coords.1 + 1).min(board.len() - 1),
            }
        }
    }
    let energized: usize = visited
        .into_iter()
        .map(|l| {
            l.into_iter()
                .filter(|v| v[0] || v[1] || v[2] || v[3])
                .count()
        })
        .sum();
    // let energized: Vec<String> = visited
    //     .into_iter()
    //     .map(|l| {
    //         l.into_iter()
    //             .map(|v| {
    //                 if v[0] || v[1] || v[2] || v[3] {
    //                     '#'
    //                 } else {
    //                     '.'
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    // for line in energized {
    //     println!("{line}");
    // }
    energized
}

pub fn part_one(input: &str) -> Option<usize> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start_coord = (0,0);
    let start_direction = Direction::Right;
    let answer = solve(&board, (start_coord, start_direction));
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rights = (0..board.len()).map(|row| ((row, 0), Direction::Right));
    let lefts = (0..board.len()).map(|row| ((row, board.len()-1), Direction::Right));
    let downs = (0..board.len()).map(|col| ((0, col), Direction::Down));
    let ups = (0..board.len()).map(|col| ((board.len()-1, col), Direction::Up));
    let mut max_energy = 0;
    for (start_coord, start_direction) in rights.chain(lefts).chain(downs).chain(ups) {
        let energy = solve(&board, (start_coord, start_direction));
        if energy > max_energy {
            max_energy = energy
        }
    }
    Some(max_energy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
