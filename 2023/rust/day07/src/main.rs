use std::cmp::Ordering;

const BITS_IN_BYTE: usize = 8;

#[cfg(feature = "part1")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(u8)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Queen,
    King,
    Ace,
}
#[cfg(feature = "part2")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(u8)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
const CARD_SIZE: usize = Card::Ace as usize + 1;

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::J,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(r#"what is that?!?!?! Got "{value}""#),
        }
    }
}

#[cfg(feature = "part1")]
impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            12 => Self::Ace,
            11 => Self::King,
            10 => Self::Queen,
            9 => Self::J,
            8 => Self::Ten,
            7 => Self::Nine,
            6 => Self::Eight,
            5 => Self::Seven,
            4 => Self::Six,
            3 => Self::Five,
            2 => Self::Four,
            1 => Self::Three,
            0 => Self::Two,
            _ => panic!(r#"what is that?!?!?! Got "{value}""#),
        }
    }
}

#[cfg(feature = "part2")]
impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            12 => Self::Ace,
            11 => Self::King,
            10 => Self::Queen,
            9 => Self::Ten,
            8 => Self::Nine,
            7 => Self::Eight,
            6 => Self::Seven,
            5 => Self::Six,
            4 => Self::Five,
            3 => Self::Four,
            2 => Self::Three,
            1 => Self::Two,
            0 => Self::J,
            _ => panic!(r#"what is that?!?!?! Got "{value}""#),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    Triplet,
    FullHouse,
    Quadruplet,
    Quintuplet,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    #[cfg(feature = "part1")]
    fn get_hand_type(&self) -> HandType {
        let mut found_indexes: [usize; CARD_SIZE] = [0; CARD_SIZE];
        for card in self.cards {
            found_indexes[card as usize] += 1;
        }
        let mut max_found = 0;
        let mut min_found = 0;
        let founds = found_indexes.iter().filter(|x| **x > 0);
        for found in founds {
            if *found > min_found {
                if *found > max_found {
                    min_found = max_found;
                    max_found = *found;
                } else {
                    min_found = *found;
                }
            }
        }
        match (max_found, min_found) {
            (5, 0) => HandType::Quintuplet,
            (4, 1) => HandType::Quadruplet,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::Triplet,
            (2, 2) => HandType::TwoPairs,
            (2, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    #[cfg(feature = "part2")]
    fn get_hand_type(&self) -> HandType {
        let mut found_indexes: [usize; CARD_SIZE] = [0; CARD_SIZE];
        for card in self.cards {
            found_indexes[card as usize] += 1;
        }
        let mut max_found = 0;
        let mut min_found = 0;
        let founds = found_indexes.iter().enumerate().filter(|(_, x)| **x > 0);
        for (card, found) in founds {
            if card == Card::J as usize {
                continue;
            }
            if *found > min_found {
                match found.cmp(&max_found) {
                    Ordering::Greater => {
                        min_found = max_found;
                        max_found = *found;
                    }
                    Ordering::Equal | Ordering::Less => {
                        min_found = *found;
                    }
                }
            }
        }
        let jokers = found_indexes[Card::J as usize];
        match (max_found + jokers, min_found) {
            (5, 0) => HandType::Quintuplet,
            (4, 1) => HandType::Quadruplet,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::Triplet,
            (2, 2) => HandType::TwoPairs,
            (2, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_value(&self) -> u64 {
        let hand_type = self.get_hand_type();
        let mut total: u64 = (hand_type as u64) << (5 * BITS_IN_BYTE);
        for (i, card) in self.cards.iter().enumerate() {
            total += (*card as u64) << ((5 - i - 1) * BITS_IN_BYTE)
        }
        total
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse().unwrap();
        if hand.len() != 5 {
            panic!("What is this hand");
        }
        let mut cards: [Card; 5] = [Card::Two; 5];
        for (i, c) in hand.chars().enumerate() {
            cards[i] = c.into();
        }
        Self { cards, bid }
    }
}

fn main() {
    let input_lines = aoc_utils::load_input_file("input.txt");
    let mut hands: Vec<_> = input_lines.map(|x| Hand::from(x.as_str())).collect();
    hands.sort();
    let result: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum();
    #[cfg(feature = "part1")]
    println!("part1 = {result}");
    #[cfg(feature = "part2")]
    println!("part2 = {result}");
}

#[cfg(test)]
mod test {
    #[cfg(feature = "part2")]
    use crate::Card;
    use crate::{Hand, HandType};
    use rstest::rstest;

    #[cfg(feature = "part1")]
    #[rstest]
    #[case("32T3K 765", HandType::OnePair)]
    #[case("T55J5 684", HandType::Triplet)]
    #[case("KK677 28", HandType::TwoPairs)]
    #[case("KTJJT 220", HandType::TwoPairs)]
    #[case("QQQJA 483", HandType::Triplet)]
    fn test_get_hand_type(#[case] input: &str, #[case] expected: HandType) {
        assert_eq!(Hand::from(input).get_hand_type(), expected);
    }

    #[cfg(feature = "part1")]
    #[rstest]
    #[case("32T3K 765", 0x01010008010B)]
    #[case("T55J5 684", 0x030803030903)]
    #[case("KK677 28", 0x020B0B040505)]
    #[case("KTJJT 220", 0x020B08090908)]
    #[case("QQQJA 483", 0x030A0A0A090C)]
    fn test_get_value(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(Hand::from(input).get_value(), expected);
    }

    #[cfg(feature = "part2")]
    #[rstest]
    #[case("32T3K 765", HandType::OnePair)]
    #[case("T55J5 684", HandType::Quadruplet)]
    #[case("KK677 28", HandType::TwoPairs)]
    #[case("KTJJT 220", HandType::Quadruplet)]
    #[case("QQQJA 483", HandType::Quadruplet)]
    fn test_get_hand_type(#[case] input: &str, #[case] expected: HandType) {
        assert_eq!(Hand::from(input).get_hand_type(), expected);
    }

    #[cfg(feature = "part2")]
    #[rstest]
    #[case("32T3K 765", 0x01020109020B)]
    #[case("T55J5 684", 0x050904040004)]
    #[case("KK677 28", 0x020B0B050606)]
    #[case("KTJJT 220", 0x050B09000009)]
    #[case("QQQJA 483", 0x050A0A0A000C)]
    fn test_get_value(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(Hand::from(input).get_value(), expected);
    }
}
