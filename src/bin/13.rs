advent_of_code::solution!(13);

#[derive(Debug)]
enum Reflection {
    Vertical(usize, usize),
    Horizontal(usize, usize),
}

fn get_cols(lines: Vec<&str>) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    let num_lines = lines.len();
    let line_length = lines[0].len();

    // Create a vector of empty strings with the same length as the input lines
    let mut transposed: Vec<String> = vec!["".to_string(); line_length];

    // Iterate over each character in each line and build the transposed lines
    for i in 0..num_lines {
        for (j, c) in lines[i].chars().enumerate() {
            transposed[j].push(c);
        }
    }
    transposed
}

fn get_reflection(rows: Vec<&str>) -> Option<Reflection> {
    // Check for horizonal reflection
    'next_row: for row in 0..rows.len()-1 {
        let mirror_row = row + 1;
        let to_check = row.min(rows.len() - mirror_row - 1);
        // println!("({row}, {mirror_row}) checking [{}, {}]", row - to_check, mirror_row + to_check);
        for offset in 0..=to_check {
            if rows[row - offset] != rows[mirror_row + offset] {
                // println!("mismatch at ({}, {})", row - offset, mirror_row + offset);
                continue 'next_row;
            }
        }
        // println!("Found horizontal reflection at ({i}, {})", i + 1);
        return Some(Reflection::Horizontal(row + 1, row + 2));
    }
    // Check for vertical reflection
    let cols = get_cols(rows);
    'next_col: for col in 0..cols.len()-1 {
        let mirror_col = col + 1;
        let to_check = col.min(cols.len() - mirror_col - 1);
        for offset in 0..=to_check {
            if cols[col - offset] != cols[mirror_col + offset] {
                continue 'next_col;
            }
        }
        return Some(Reflection::Vertical(col + 1, col + 2));
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total_cols = 0;
    let mut total_rows = 0;
    for pattern in input.split("\n\n") {
        let reflection = get_reflection(pattern.lines().collect());
        match reflection {
            Some(Reflection::Vertical(cols, _)) => total_cols += cols,
            Some(Reflection::Horizontal(rows, _)) => total_rows += rows,
            None => unreachable!(),
        }
    }
    Some(total_cols + (total_rows * 100))
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
