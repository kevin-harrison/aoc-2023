advent_of_code::solution!(12);
use cached::proc_macro::cached;

#[cached]
fn solve_one(record: Vec<char>, start: usize, damaged_springs: Vec<usize>, spring: usize) -> u64 {
    let mut count = 0;
    let consecutive_damaged_springs = damaged_springs[spring];
    for i in start..record.len() {
        let spot_after = i + consecutive_damaged_springs;
        if spot_after > record.len() {
            break;
        }
        // Cant place space before springs
        if i > 0 && (record[i - 1] != '.' && record[i - 1] != '?') {
            // println!("skipping {i} no place for before spot.");
            if record[i] == '#' {
                break;
            }
            continue;
        }
        // Cant place springs
        if record[i..spot_after]
            .into_iter()
            .any(|&c| c != '#' && c != '?')
        {
            // println!("skipping {i} no place for springs.");
            if record[i] == '#' {
                break;
            }
            continue;
        }
        // Cant place space after
        if spot_after != record.len() && (record[spot_after] != '.' && record[spot_after] != '?') {
            // println!("skipping {i} no place for after spot.");
            if record[i] == '#' {
                break;
            }
            continue;
        }
        // Spring place-able!
        let next_start = i + consecutive_damaged_springs + 1;
        let rest = if spring == damaged_springs.len() - 1 {
            if next_start < record.len() && record[next_start..].iter().any(|&c| c == '#') {
                0
            } else {
                1
            }
        } else {
            solve_one(
                record.clone(),
                next_start,
                damaged_springs.clone(),
                spring + 1,
            )
        };
        count += rest;
        // if rest > 0 {
        //     println!("{} placed at {i} gives {rest} combinations", spring);
        // }
        if record[i] == '#' {
            break;
        }
    }
    count
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (record_str, springs_str) = line.split_once(" ").unwrap();
    let record = record_str.chars().collect();
    let damaged_springs = springs_str
        .split(",")
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (record, damaged_springs)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0;
    for line in input.lines() {
        let (record, damaged_springs) = parse_line(line);
        let sol = solve_one(record, 0, damaged_springs, 0);
        answer += sol;
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut answer = 0;
    for line in input.lines() {
        let (mut record, damaged_springs) = parse_line(line);
        record.push('?');
        let records = record.len();
        let springs = damaged_springs.len();
        let unfolded_record: Vec<char> =
            record.into_iter().cycle().take((5 * records) - 1).collect();
        let unfolded_springs = damaged_springs
            .into_iter()
            .cycle()
            .take(5 * springs)
            .collect();
        let sol = solve_one(unfolded_record, 0, unfolded_springs, 0);
        answer += sol;
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
