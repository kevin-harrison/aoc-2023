advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();

    let mut answer = 1;
    for i in 0..times.len() {
        let distance_to_beat = distances[i];
        let mut ways_to_beat = 0;
        for hold_time in 0..=times[i] {
            let distance_traveled = hold_time * (times[i] - hold_time);
            if distance_traveled > distance_to_beat {
                ways_to_beat += 1;
            }
        }
        answer *= ways_to_beat
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let discriminant = ((time * time) - (4_f64 * distance)).sqrt();
    let mut root1 = (-time + discriminant) / -2_f64;
    let mut root2 = (-time - discriminant) / -2_f64;
    if root1 > root2 {
        (root1, root2) = (root2, root1);
    }
    Some((root2 - root1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
