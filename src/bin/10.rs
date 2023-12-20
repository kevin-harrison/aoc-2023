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

    // Hard coded to move down on first move
    let mut dist = 0;
    let mut curr_row = start_row + 1;
    let mut curr_col = start_col;
    let mut prev_dir = Direction::Down;
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
    Some(dist / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
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

    // Hard coded to move down on first move
    let mut pipe_path = vec![Vec::new(); map.len()];
    let mut curr_row = start_row + 1;
    let mut curr_col = start_col;
    let mut prev_dir = Direction::Down;
    loop {
        pipe_path[curr_row].push(curr_col);
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

    let mut interior_points = 0;
    for row in 0..map.len() {
        let mut intersections = 0;
        let mut bend_start = 'n';
        for col in 0..map[0].len() {
            if pipe_path[row].contains(&col) {
                match map[row][col] {
                    '|' => intersections += 1,
                    '-' => (),
                    'J' => {
                        if bend_start == 'F' || bend_start == 'S' {
                            intersections += 1;
                        } else if bend_start == 'L' {
                            intersections += 2;
                        } else {
                            unreachable!();
                        }
                    }
                    '7' => {
                        if bend_start == 'F' || bend_start == 'S' {
                            intersections += 2;
                        } else if bend_start == 'L' {
                            intersections += 1;
                        } else {
                            unreachable!();
                        }
                    }
                    bend => bend_start = bend,
                }
                continue;
            }
            if (intersections % 2) == 1 {
                interior_points += 1;
            }
        }
    }
    Some(interior_points)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
