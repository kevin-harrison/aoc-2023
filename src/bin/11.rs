use itertools::Itertools;
advent_of_code::solution!(11);

fn solution(input: &str, expansion: usize) -> usize {
    // Get stars and expand rows
    let mut stars = vec![];
    let mut row = 0;
    for line in input.lines() {
        let mut empty_row = true;
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                empty_row = false;
                stars.push((col, row));
            }
        }
        if empty_row {
            row += expansion;
        } else {
            row += 1;
        }
    }

    // Expand cols
    stars.sort();
    let mut col_dialation = 0;
    let mut last_star_col = 0;
    for (star_col, _) in stars.iter_mut() {
        let cols_passed = *star_col - last_star_col;
        if cols_passed > 1 {
            col_dialation += (cols_passed - 1) * (expansion - 1);
        }
        last_star_col = *star_col;
        *star_col += col_dialation;
    }

    // Sum star distances
    let mut total_dist = 0;
    for pair in stars.iter().combinations(2) {
        total_dist += pair[1].0 - pair[0].0;
        total_dist += (pair[0].1 as i64 - pair[1].1 as i64).abs() as usize;
    }
    total_dist
}

pub fn part_one(input: &str) -> Option<usize> {
    let answer = solution(input, 2);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let answer = solution(input, 10);
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
