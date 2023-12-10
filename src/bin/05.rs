advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Translation {
    source: u64,
    destination: u64,
    range: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Translation>>) {
    let mut paragraphs = input.split("\n\n");
    let seeds: Vec<u64> = paragraphs
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|seed_str| seed_str.parse::<u64>().unwrap())
        .collect();
    let transforms: Vec<Vec<Translation>> = paragraphs
        .map(|paragaph| {
            paragaph
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line
                        .split(" ")
                        .map(|num_str| num_str.parse::<u64>().unwrap());
                    Translation {
                        destination: nums.next().unwrap(),
                        source: nums.next().unwrap(),
                        range: nums.next().unwrap(),
                    }
                })
                .collect()
        })
        .collect();
    (seeds, transforms)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, transforms) = parse_input(input);
    // println!("Seeds ={seeds:?}");
    // println!("transforms ={transforms:#?}");

    let mut locations = vec![];
    for mut seed in seeds {
        for transform in transforms.iter() {
            for translation in transform.iter() {
                let translate_range_start = translation.source;
                let translate_range_end = translation.source + translation.range;
                if seed >= translate_range_start && seed < translate_range_end {
                    // number is inside the tranlation range
                    seed = translation.destination + (seed - translate_range_start);
                    break;
                }
            }
        }
        locations.push(seed);
    }
    // println!("Locations ={locations:?}");
    Some(locations.into_iter().min().unwrap())
}

fn translate_intervals(intervals: Vec<Range>, transform: &Vec<Translation>) -> Vec<Range> {
    let mut translated_intervals = vec![];
    for mut range in intervals {
        let mut entire_range_translated = false;
        for translation in transform.iter() {
            let translate_range_start = translation.source;
            let translate_range_end = translation.source + translation.range;
            // println!("[{},{}) int [{},{})?", range.start, range.end, translate_range_start, translate_range_end);
            if !(range.start > translate_range_end || translate_range_start > range.end) {
                let overlap_start = range.start.max(translate_range_start);
                let overlap_end = range.end.min(translate_range_end);
                // println!("Found overlap ({overlap_start}, {overlap_end})");
                if overlap_start != range.start {
                    translated_intervals.push(Range {
                        start: range.start,
                        end: overlap_start,
                    });
                }
                translated_intervals.push(Range {
                    start: (overlap_start - translate_range_start) + translation.destination,
                    end: (overlap_end - translate_range_start) + translation.destination,
                });
                if overlap_end != range.end {
                    range.start = overlap_end;
                } else {
                    // println!("break");
                    entire_range_translated = true;
                    break;
                }
            }
        }
        if !entire_range_translated {
            translated_intervals.push(range);
        }
    }
    translated_intervals
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mut transforms) = parse_input(input);
    let seed_ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|str_pair| Range {
            start: str_pair[0],
            end: str_pair[0] + str_pair[1],
        })
        .collect();
    for layer in transforms.iter_mut() {
        layer.sort();
    }
    // println!("Seeds ={seed_ranges:?}");
    // println!("Transforms ={:?}", transforms[0]);

    let mut min_locations: Vec<Range> = vec![];
    for range in seed_ranges {
        let mut intervals = vec![range];
        // println!("INTERVALS = {intervals:?}");
        for transform in transforms.iter() {
            intervals = translate_intervals(intervals, transform);
            // println!("INTERVALS = {intervals:?}");
        }
        let min_internal = intervals.iter().min().unwrap();
        min_locations.push((*min_internal).clone());
        // println!("\n\nNext seed range:");
    }
    let min_range = min_locations.iter().min().unwrap();
    Some(min_range.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
