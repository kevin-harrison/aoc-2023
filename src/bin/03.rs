advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<(usize, &str)> = input.trim_end().lines().enumerate().collect();
    let num_lines = lines.len();
    let mut nums: Vec<(usize, usize, &str)> = vec![];
    let mut symbol_coords: Vec<Vec<usize>> = vec![Vec::new(); num_lines];

    let mut num_start = None;
    for (row, line) in lines {
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => (),
                c if c.is_numeric() => (),
                _ => symbol_coords[row].push(col),
            }
            match (char.is_numeric(), num_start) {
                // Found start of number
                (true, None) => num_start = Some(col),
                // Found end of number
                (false, Some(idx)) => {
                    nums.push((row, idx, &line[idx..col]));
                    num_start = None;
                }
                _ => (),
            }
        }
        if let Some(idx) = num_start {
            nums.push((row, idx, &line[idx..]));
            num_start = None;
        }
    }

    let mut sum = 0;
    for (row, col, number_str) in nums {
        let left_bound = col.saturating_sub(1);
        let right_bound = (col + 1 + number_str.len()).min(num_lines);

        for i in row.saturating_sub(1)..(row + 2).min(num_lines) {
            if symbol_coords[i]
                .iter()
                .any(|&col| col >= left_bound && col < right_bound)
            {
                sum += number_str.parse::<u32>().unwrap();
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<(usize, &str)> = input.trim_end().lines().enumerate().collect();
    let num_lines = lines.len();
    let mut gear_coords: Vec<(usize, usize)> = vec![];
    let mut nums: Vec<Vec<(usize, &str)>> = vec![Vec::new(); num_lines];

    let mut num_start = None;
    for (row, line) in lines {
        for (col, char) in line.chars().enumerate() {
            match char {
                '*' => gear_coords.push((row, col)),
                _ => (),
            }
            match (char.is_numeric(), num_start) {
                // Found start of number
                (true, None) => num_start = Some(col),
                // Found end of number
                (false, Some(idx)) => {
                    nums[row].push((idx, &line[idx..col]));
                    num_start = None;
                }
                _ => (),
            }
        }
        if let Some(idx) = num_start {
            nums[row].push((idx, &line[idx..]));
            num_start = None;
        }
    }

    let mut sum = 0;
    for (row, col) in gear_coords {
        let left_bound = col.saturating_sub(1);
        let right_bound = (col + 2).min(num_lines);

        let mut adjacent_words = vec![];
        for i in row.saturating_sub(1)..(row + 2).min(num_lines) {
            let adjacent_on_row = nums[i]
                .iter()
                .filter(|(num_start, num_str)| {
                    (*num_start >= left_bound && *num_start < right_bound)
                        || (*num_start < left_bound && *num_start + num_str.len() - 1 >= left_bound)
                })
                .map(|(_, num_str)| num_str.parse::<u32>().unwrap());
            adjacent_words.extend(adjacent_on_row);
            if adjacent_words.len() > 2 {
                break;
            }
        }
        if adjacent_words.len() == 2 {
            sum += adjacent_words.iter().product::<u32>();
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
