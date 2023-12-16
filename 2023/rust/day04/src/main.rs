use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: BTreeSet<u32>,
    numbers_drawn: BTreeSet<u32>,
}

impl Card {
    fn get_total_wins(&self) -> usize {
        let winnings: Vec<&u32> = self
            .winning_numbers
            .intersection(&self.numbers_drawn)
            .collect();
        winnings.len()
    }
    fn get_total_points(&self) -> usize {
        let winnings = self.get_total_wins();
        if winnings > 0 {
            2_usize.pow(winnings as u32 - 1)
        } else {
            0
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':');
        let card_id = parts
            .next()
            .ok_or("game format not compliant".to_owned())?
            .trim();
        let card_id = card_id
            .split_whitespace()
            .last()
            .ok_or("card id not found".to_owned())?
            .parse::<u32>()
            .map_err(|_| "couldn't parse card id".to_owned())?;
        let mut numbers = parts
            .next()
            .ok_or("card format not compliant".to_owned())?
            .split('|');
        let winning_numbers = numbers
            .next()
            .ok_or("winning numbers not compliant".to_owned())?;
        let numbers = numbers.next().ok_or("numbers not compliant".to_owned())?;

        Ok(Self {
            id: card_id,
            winning_numbers: winning_numbers
                .split_whitespace()
                .flat_map(str::parse::<u32>)
                .collect::<BTreeSet<u32>>(),
            numbers_drawn: numbers
                .split_whitespace()
                .flat_map(str::parse::<u32>)
                .collect::<BTreeSet<u32>>(),
        })
    }
}

fn main() {
    let input_lines = aoc_utils::load_input_file("input.txt");
    let cards: Vec<Card> = input_lines
        .flat_map(|line| Card::try_from(line.as_str()))
        .collect();
    let winnings_sum: usize = cards.iter().map(Card::get_total_points).sum();

    let multipliers_len = cards.len();
    let mut multipliers = vec![1_u64; multipliers_len];
    for (indx, card) in cards.iter().enumerate() {
        let start_slice = indx + 1;
        let end_slice = start_slice + card.get_total_wins();
        let end_slice = end_slice.min(multipliers_len);
        println!("start: {start_slice}  end:{end_slice}");
        let curr_multiplier = multipliers[indx];
        for multiplier in &mut multipliers[start_slice..end_slice] {
            *multiplier += curr_multiplier;
        }
    }
    let multipliers_sum: u64 = multipliers.iter().sum();

    println!("part1: {winnings_sum}");
    println!("part2: {multipliers_sum}");
}

#[cfg(test)]
mod tests {
    use super::{BTreeSet, Card};
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",Ok(Card{id:1,winning_numbers:BTreeSet::from([41,48,83,86,17]),numbers_drawn:BTreeSet::from([83,86,6,31,17,9,48,53])}))]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",Ok(Card{id:2,winning_numbers:BTreeSet::from([13,32,20,16,61]),numbers_drawn:BTreeSet::from([61,30,68,82,17,32,24,19])}))]
    fn test_card_parse(#[case] input: &str, #[case] expected: Result<Card, String>) {
        assert_eq!(Card::try_from(input), expected);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_get_card_points(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(Card::try_from(input).unwrap().get_total_points(), expected);
    }
}
