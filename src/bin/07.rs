advent_of_code::solution!(7);

type Bid = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart1 {
    Ace,
    Knight,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart2 {
    Ace,
    Knight,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    CardPart1(CardPart1),
    CardPart2(CardPart2),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct HandCards {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
    fifth: Card,
}

impl HandCards {
    fn from(cards: [Card; 5]) -> Self {
        Self {
            first: cards[0],
            second: cards[1],
            third: cards[2],
            fourth: cards[3],
            fifth: cards[4],
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    FiveOfAKind(HandCards),
    FourOfAKind(HandCards),
    FullHouse(HandCards),
    ThreeOfAKind(HandCards),
    TwoPair(HandCards),
    OnePair(HandCards),
    HighCard(HandCards),
}

#[derive(Debug)]
struct ParseHandError;
impl Hand {
    fn from_str(s: &str, part2: bool) -> Result<Self, ParseHandError> {
        if s.chars().count() != 5 {
            return Err(ParseHandError);
        }
        let mut cards = [Card::CardPart1(CardPart1::Two); 5];
        let mut counts = [0; 13];
        let mut joker_count = 0;
        for (i, c) in s.chars().enumerate() {
            let card = match c {
                'A' => {
                    counts[0] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Ace))
                    } else {
                        Some(Card::CardPart1(CardPart1::Ace))
                    }
                }
                'K' => {
                    counts[1] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Knight))
                    } else {
                        Some(Card::CardPart1(CardPart1::Knight))
                    }
                }
                'Q' => {
                    counts[2] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Queen))
                    } else {
                        Some(Card::CardPart1(CardPart1::Queen))
                    }
                }
                'J' => {
                    if part2 {
                        joker_count += 1;
                        Some(Card::CardPart2(CardPart2::Joker))
                    } else {
                        counts[3] += 1;
                        Some(Card::CardPart1(CardPart1::Jack))
                    }
                }
                'T' => {
                    counts[4] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Ten))
                    } else {
                        Some(Card::CardPart1(CardPart1::Ten))
                    }
                }
                '9' => {
                    counts[5] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Nine))
                    } else {
                        Some(Card::CardPart1(CardPart1::Nine))
                    }
                }
                '8' => {
                    counts[6] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Eight))
                    } else {
                        Some(Card::CardPart1(CardPart1::Eight))
                    }
                }
                '7' => {
                    counts[7] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Seven))
                    } else {
                        Some(Card::CardPart1(CardPart1::Seven))
                    }
                }
                '6' => {
                    counts[8] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Six))
                    } else {
                        Some(Card::CardPart1(CardPart1::Six))
                    }
                }
                '5' => {
                    counts[9] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Five))
                    } else {
                        Some(Card::CardPart1(CardPart1::Five))
                    }
                }
                '4' => {
                    counts[10] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Four))
                    } else {
                        Some(Card::CardPart1(CardPart1::Four))
                    }
                }
                '3' => {
                    counts[11] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Three))
                    } else {
                        Some(Card::CardPart1(CardPart1::Three))
                    }
                }
                '2' => {
                    counts[12] += 1;
                    if part2 {
                        Some(Card::CardPart2(CardPart2::Two))
                    } else {
                        Some(Card::CardPart1(CardPart1::Two))
                    }
                }
                _ => None,
            };
            match card {
                Some(parsed_card) => cards[i] = parsed_card,
                None => return Err(ParseHandError),
            }
        }
        counts.sort();
        let mut top_counts: Vec<usize> = counts.into_iter().rev().take(2).collect();
        if part2 {
            top_counts[0] += joker_count;
        }
        let hand = match (top_counts[0], top_counts[1]) {
            (5, 0) => Hand::FiveOfAKind(HandCards::from(cards)),
            (4, 1) => Hand::FourOfAKind(HandCards::from(cards)),
            (3, 2) => Hand::FullHouse(HandCards::from(cards)),
            (3, 1) => Hand::ThreeOfAKind(HandCards::from(cards)),
            (2, 2) => Hand::TwoPair(HandCards::from(cards)),
            (2, 1) => Hand::OnePair(HandCards::from(cards)),
            (1, 1) => Hand::HighCard(HandCards::from(cards)),
            (a, b) => unreachable!("({a}, {b}) hand combo impossible."),
        };
        Ok(hand)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut data: Vec<(Hand, Bid)> = vec![];
    for line in input.lines() {
        let (hand_str, bid_str) = line.split_once(' ').unwrap();
        let hand = Hand::from_str(hand_str, false).unwrap();
        let bid = bid_str.parse::<Bid>().unwrap();
        data.push((hand, bid));
    }
    data.sort_by(|a, b| b.0.cmp(&a.0));
    let answer: usize = data
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();
    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data: Vec<(Hand, Bid)> = vec![];
    for line in input.lines() {
        let (hand_str, bid_str) = line.split_once(' ').unwrap();
        let hand = Hand::from_str(hand_str, true).unwrap();
        let bid = bid_str.parse::<Bid>().unwrap();
        data.push((hand, bid));
    }
    data.sort_by(|a, b| b.0.cmp(&a.0));
    let answer: usize = data
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();
    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
