use num::integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    // Build directions
    let directions_str = lines.next().unwrap();
    let directions: Vec<usize> = directions_str
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!("unrecognized direction"),
        })
        .collect();

    // Build graph
    lines.next();
    let mut labels = HashMap::new();
    let mut graph = vec![];
    let mut start = 0;
    let mut end = 0;
    for (i, line) in lines.enumerate() {
        let label = &line[0..3];
        if label == "AAA" {
            start = i;
        } else if label == "ZZZ" {
            end = i;
        }
        let left_label = &line[7..10];
        let right_label = &line[12..15];
        labels.insert(label, (i, left_label, right_label));
        graph.push([0, 0]);
    }
    // println!("{labels:?}");
    for (_, &(index, left_label, right_label)) in labels.iter() {
        let left_index = labels.get(left_label).unwrap().0;
        let right_index = labels.get(right_label).unwrap().0;
        graph[index] = [left_index, right_index];
    }
    // println!("start={start}, end={end}");
    // println!("{graph:?}");

    let mut i = 0;
    let mut curr = start;
    while curr != end {
        curr = graph[curr][directions[i % directions.len()]];
        i += 1;
    }
    Some(i as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    // Build directions
    let directions_str = lines.next().unwrap();
    let directions: Vec<usize> = directions_str
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!("unrecognized direction"),
        })
        .collect();

    // Build graph
    lines.next();
    let mut labels = HashMap::new();
    let mut graph = vec![];
    let mut starts = vec![];
    let mut ends = vec![];
    for (i, line) in lines.enumerate() {
        let label = &line[0..3];
        if label.ends_with('A') {
            starts.push(i);
        } else if label.ends_with('Z') {
            ends.push(i);
        }
        let left_label = &line[7..10];
        let right_label = &line[12..15];
        labels.insert(label, (i, left_label, right_label));
        graph.push([0, 0]);
    }
    for (_, &(index, left_label, right_label)) in labels.iter() {
        let left_index = labels.get(left_label).unwrap().0;
        let right_index = labels.get(right_label).unwrap().0;
        graph[index] = [left_index, right_index];
    }

    // Calc cycle lengths
    let mut cycles = vec![];
    for mut curr in starts {
        let mut i = 0;
        while !ends.contains(&curr) {
            curr = graph[curr][directions[i % directions.len()]];
            i += 1;
        }
        // NOTE: we dont need to worry about the cycle lengths as they happen to match up with the
        // cycle starts.
        cycles.push(i);
    }
    let total_lcm = cycles.into_iter().fold(1, |acc, cycle| lcm(acc, cycle));
    Some(total_lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
