use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [u32; 5],
    bet: u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_hand(line: &str) -> Hand {
    let (hand, bet) = line.split_once(" ").unwrap();
    let mut cards = [0; 5];
    for (i, card) in hand.chars().enumerate() {
        cards[i] = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        };
    }
    let res = cards.iter().counts();
    let counts = res.values().sorted().rev().collect_vec();

    let hand_type = match counts.as_slice() {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [2, 1, 1, 1] => HandType::OnePair,
        [2, 2, 1] => HandType::TwoPairs,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [3, 2] => HandType::FullHouse,
        [4, 1] => HandType::FourOfAKind,
        [5] => HandType::FiveOfAKind,
        _ => panic!("Invalid hand"),
    };
    // cards.sort();
    Hand {
        hand_type,
        cards,
        bet: bet.parse().unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut hands: Vec<Hand> = lines.map(parse_hand).collect();
    hands.sort();
    hands.iter().enumerate().fold(Some(0), |acc, (i, hand)| {
        let value = acc.unwrap();
        Some(value + hand.bet * (i as u32 + 1))
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
