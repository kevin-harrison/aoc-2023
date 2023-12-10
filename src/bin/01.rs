advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim_end();
    let mut sum = 0;
    for line in input.split("\n") {
        let mut digits = line.chars().filter(|char| char.is_numeric());
        let first_digit = digits.next().expect("Didn't find a first digit on a line.");
        let last_digit = match digits.last() {
            Some(digit) => digit,
            None => first_digit,
        };
        let num = first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
        sum += num;
    }
    Some(sum)
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGIT_WORDS_BACKWARDS: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn get_first_digit(line: &str) -> u32 {
    let mut chars = line.chars();
    loop {
        let peeked_chars: String = chars.clone().take(5).collect();
        for (i, word) in DIGIT_WORDS.iter().enumerate() {
            if peeked_chars.starts_with(word) {
                return i as u32 + 1;
            }
        }
        match chars.next() {
            Some(char) if char.is_numeric() => return char.to_digit(10).unwrap(),
            None => unreachable!("Didn't find first digit on line."),
            _ => (),
        }
    }
}

fn get_last_digit(line: &str) -> u32 {
    let mut chars = line.chars().rev();
    loop {
        let peeked_chars: String = chars.clone().take(5).collect();
        for (i, word) in DIGIT_WORDS_BACKWARDS.iter().enumerate() {
            if peeked_chars.starts_with(word) {
                return i as u32 + 1;
            }
        }
        match chars.next() {
            Some(char) if char.is_numeric() => return char.to_digit(10).unwrap(),
            None => unreachable!("Didn't find first digit on line."),
            _ => (),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim_end();
    let mut sum = 0;
    for line in input.split("\n") {
        let first_digit = get_first_digit(line);
        let last_digit = get_last_digit(line);
        let line_num = first_digit * 10 + last_digit;
        sum += line_num;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
