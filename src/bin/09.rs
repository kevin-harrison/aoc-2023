advent_of_code::solution!(9);

fn next_seq_num(seq: Vec<i64>) -> i64 {
    if seq.iter().all(|n| *n == 0) {
        0
    } else {
        let last_in_seq = seq[seq.len() - 1];
        let diff_seq = seq.windows(2).map(|win| win[1] - win[0]).collect();
        last_in_seq + next_seq_num(diff_seq)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut answer = 0;
    for line in input.lines() {
        let seq: Vec<i64> = line
            .split(" ")
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        let next_in_seq = next_seq_num(seq);
        answer += next_in_seq;
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut answer = 0;
    for line in input.lines() {
        let seq: Vec<i64> = line
            .split(" ")
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        let next_in_seq = next_seq_num(seq.into_iter().rev().collect());
        answer += next_in_seq;
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
