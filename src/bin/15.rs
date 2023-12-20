advent_of_code::solution!(15);

fn hash(key: &str) -> usize {
    let mut hash = 0;
    for ascii in key.chars().map(|c| c as u8) {
        hash += ascii as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

pub fn part_one(input: &str) -> Option<usize> {
    let sequence = input.chars().filter(|c| *c != '\n').collect::<String>();
    let sequence: Vec<&str> = sequence.split(",").collect();
    let mut answer = 0;
    for word in sequence {
        answer += hash(word);
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequence = input.chars().filter(|c| *c != '\n').collect::<String>();
    let sequence: Vec<&str> = sequence.split(",").collect();
    let mut boxes = vec![Vec::new(); 256];
    for word in sequence {
        match word.split_once('=') {
            Some((key, val)) => {
                let key_idx = hash(key);
                let val = val.parse::<usize>().unwrap();
                if let Some((_, prev_val)) = boxes[key_idx].iter_mut().find(|(k, _)| *k == key) {
                    *prev_val = val;
                } else {
                    boxes[key_idx].push((key, val));
                }
            }
            None => {
                let remove_key = &word[..word.len() - 1];
                let key_idx = hash(remove_key);
                if let Some(index) = boxes[key_idx].iter().position(|(k, _)| *k == remove_key) {
                    boxes[key_idx].remove(index);
                }
            }
        }
    }
    let mut answer = 0;
    for (box_num, b) in boxes.into_iter().enumerate() {
        // if !b.is_empty() {
        //     println!("Box {box_num}: {b:?}");
        // }
        for (box_idx, lens) in b.into_iter().enumerate() {
            answer += (box_num + 1) * (box_idx + 1) * lens.1;
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
