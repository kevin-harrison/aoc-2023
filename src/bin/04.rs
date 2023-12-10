use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.trim_end().lines() {
        let colon_index = line.find(":").unwrap();
        let (winning_str, my_nums_str) = line[colon_index + 2..].split_once(" | ").unwrap();
        let winning_nums = winning_str
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let my_nums = my_nums_str
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let num_of_wins = winning_nums.intersection(&my_nums).count();
        if num_of_wins > 0 {
            sum += 1 << num_of_wins - 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points = vec![];
    for line in input.trim_end().lines() {
        let colon_index = line.find(":").unwrap();
        let (winning_str, my_nums_str) = line[colon_index + 2..].split_once(" | ").unwrap();
        let winning_nums = winning_str
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let my_nums = my_nums_str
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let num_of_wins = winning_nums.intersection(&my_nums).count();
        points.push(num_of_wins);
    }
    let num_cards = points.len();

    let mut cards_copies = vec![1; num_cards];
    for card in 0..num_cards {
        for activate in card + 1..card + points[card] + 1 {
            cards_copies[activate] += cards_copies[card];
        }
    }
    Some(cards_copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
