#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    None,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
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

enum HandType {
    None,
    Pair(Card),
    TwoPairs((Card, Card)),
    Triplet(Card),
    Quadruplet(Card),
    FullHouse((Card, Card)),
}

struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let bid = parts.next().unwrap().parse().unwrap();
        let hand = parts.next().unwrap();
        if hand.len() != 5 {
            panic!("What is this hand");
        }
        let mut cards: [Card; 5] = [Card::None; 5];
        for (i, c) in hand.chars().enumerate() {
            cards[i] = c.into();
        }
        Self { cards, bid }
    }
}

fn main() {
    let input_lines = aoc_utils::load_input_file("test.txt");
    let hands: Vec<_> = input_lines.map(|x| Hand::from(x.as_str())).collect();
}
