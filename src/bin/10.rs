advent_of_code::solution!(10);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = vec![];
    let mut start_row = 0;
    let mut start_col = 0;
    for (row, line) in input.lines().enumerate() {
        if let Some(col) = line.find('S') {
            start_row = row;
            start_col = col;
        }
        map.push(line.chars().collect());
    }
    // println!("{start_row}, {start_col}");
    // println!("{map:?}");

    // Hard coded to move down on first move
    let mut dist = 0;
    let mut curr_row = start_row+1;
    let mut curr_col = start_col;
    let mut prev_dir = Direction::Down;
    // while curr_row != start_row || curr_col != start_col {
    loop {
        dist += 1;
        match (map[curr_row][curr_col], prev_dir) {
            ('S', _) => break,
            ('L', Direction::Down) => prev_dir = Direction::Right,
            ('L', Direction::Left) => prev_dir = Direction::Up,
            ('J', Direction::Down) => prev_dir = Direction::Left,
            ('J', Direction::Right) => prev_dir = Direction::Up,
            ('7', Direction::Up) => prev_dir = Direction::Left,
            ('7', Direction::Right) => prev_dir = Direction::Down,
            ('F', Direction::Up) => prev_dir = Direction::Right,
            ('F', Direction::Left) => prev_dir = Direction::Down,
            _ => (),
        }
        match prev_dir {
            Direction::Up => curr_row -= 1,
            Direction::Down => curr_row += 1,
            Direction::Left => curr_col -= 1,
            Direction::Right => curr_col += 1,
        }
    }
    // println!("{dist}");
    Some(dist / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
