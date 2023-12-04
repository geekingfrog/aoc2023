use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

pub fn solve() -> (usize, usize) {
    let cards = parse_cards(include_str!("../resources/day04.txt"));
    let part1 = cards.iter().map(|c| c.points()).sum();
    (part1, part2(&cards))
}

fn part2(cards: &[Card]) -> usize {
    let mut card_count = vec![1; cards.len()];
    for card in cards {
        let l = card
            .nums
            .iter()
            .filter(|x| card.winning.contains(x))
            .count();
        for j in 0..l {
            card_count[j + card.id] += card_count[card.id - 1];
        }
    }
    card_count.into_iter().sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: usize,
    winning: Vec<u8>,
    nums: Vec<u8>,
}

impl Card {
    fn points(&self) -> usize {
        let l = self
            .nums
            .iter()
            .filter(|x| self.winning.contains(x))
            .count();
        if l == 0 {
            0
        } else {
            1 << (l - 1)
        }
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    let (_rest, cards) = separated_list1(line_ending, parse_line)(input)
        .finish()
        .expect("parse input");
    cards
}

fn parse_line(line: &str) -> IResult<&str, Card> {
    map(
        tuple((
            parse_id,
            separated_list1(space1, complete::u8),
            tuple((space1, complete::char('|'), space1)),
            separated_list1(space1, complete::u8),
        )),
        |(id, winning, _, nums)| Card { id, winning, nums },
    )(line)
}

fn parse_id(input: &str) -> IResult<&str, usize> {
    map(
        delimited(
            tag("Card"),
            preceded(space1, complete::u64),
            tuple((complete::char(':'), space1)),
        ),
        |x| x as _,
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_card_point() {
        let raw = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = &parse_cards(raw)[0];
        assert_eq!(card.points(), 8)
    }

    #[test]
    fn test_part2() {
        let raw = include_str!("../resources/day04_test.txt");
        let cards = parse_cards(raw);
        assert_eq!(part2(&cards), 30);
    }
}
