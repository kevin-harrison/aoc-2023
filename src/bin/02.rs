advent_of_code::solution!(2);

#[derive(Debug)]
struct GameResult {
    red: u32,
    blue: u32,
    green: u32,
}

impl GameResult {
    pub fn from(game_string: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for cube_result in game_string.split(", ") {
            let (amount_str, color) = cube_result.split_once(" ").unwrap();
            let amount: u32 = amount_str.parse().unwrap();
            match color {
                "red" => red += amount,
                "blue" => blue += amount,
                "green" => green += amount,
                _ => unreachable!("Unrecognized cube color."),
            }
        }
        Self { red, blue, green }
    }
}

const MAX_RED_CUBES: u32 = 12;
const MAX_BLUE_CUBES: u32 = 14;
const MAX_GREEN_CUBES: u32 = 13;

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim_end();
    let mut sum = 0;
    for line in input.split("\n") {
        let colon_index = line.find(':').unwrap();
        let game_id: u32 = line[0..colon_index]
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let game_results: Vec<GameResult> = line[colon_index + 2..]
            .split("; ")
            .map(|game_str| GameResult::from(game_str))
            .collect();
        let mut found_impossible_game = false;
        for game in game_results {
            if game.red > MAX_RED_CUBES
                || game.blue > MAX_BLUE_CUBES
                || game.green > MAX_GREEN_CUBES
            {
                found_impossible_game = true;
                break;
            }
        }
        if !found_impossible_game {
            sum += game_id
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim_end();
    let mut sum = 0;
    for line in input.split("\n") {
        let colon_index = line.find(':').unwrap();
        let game_results: Vec<GameResult> = line[colon_index + 2..]
            .split("; ")
            .map(|game_str| GameResult::from(game_str))
            .collect();
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for game in game_results {
            if game.red > max_red {
                max_red = game.red;
            }
            if game.blue > max_blue {
                max_blue = game.blue;
            }
            if game.green > max_green {
                max_green = game.green;
            }
        }
        let power = max_red * max_blue * max_green;
        sum += power;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
