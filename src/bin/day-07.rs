pub fn main() {
    let data = include_str!("../../data/07.in");
    println!("part 1: {}", solve(data, |hand1, hand2| hand1.cmp_part1(hand2)));
    println!("part 2: {}", solve(data, |hand1, hand2| hand1.cmp_part2(hand2)));
}

pub fn solve<F>(data: &str, cmp: F) -> usize 
where
    F: Fn(&Hand, &Hand) -> Option<Ordering>
{

    let mut hands: Vec<_> =

    data
    .lines()
    .map(|line| {
        Hand::new(line)
    }).collect();

    hands.sort_by(|hand1, hand2| cmp(hand1, hand2).unwrap());

    hands
    .iter()
    .enumerate()
    .map(|(idx, hand)| {
        (idx + 1) * hand.bid
    })
    .sum()
}

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    bid: usize,
    counts: HashMap<Card, usize>
}


impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = write!(f, "(");
        for card in self.cards {
            _ = write!(f, "{}", card);
        }
        _ = write!(f, ", ");
        _ = write!(f, "{}, {:?}", self.bid, self.kind_part2());
        write!(f, ")")
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    #[default]
    HighCard = 1
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Num(u8),
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "{}", 'A'),
            Self::K => write!(f, "{}", 'K'),
            Self::Q => write!(f, "{}", 'Q'),
            Self::J => write!(f, "{}", 'J'),
            Self::T => write!(f, "{}", 'T'),
            Self::Num(digit) => write!(f, "{}", digit)
        }
    }
}


impl TryFrom<char> for Card {
    type Error = std::io::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            digit @ '0'..='9' => Ok(Card::Num(digit.to_digit(10).unwrap() as u8)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "unknown card type."))
        }
    }
}


impl Hand {
    fn cmp_part1(&self, other: &Self) -> Option<std::cmp::Ordering> {

        let own_kind = self.kind_part1();
        let other_kind = other.kind_part1();

        if own_kind != other_kind {
            return own_kind.partial_cmp(&other_kind)
        }

        for (own_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if own_card != other_card {
                return own_card.partial_cmp(other_card);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }


    fn cmp_part2(&self, other: &Self) -> Option<std::cmp::Ordering> {

        let own_kind = self.kind_part2();
        let other_kind = other.kind_part2();

        if own_kind != other_kind {
            return own_kind.partial_cmp(&other_kind)
        }

        for (own_card, other_card) in self.cards.iter().zip(other.cards.iter()) {

            match (*own_card, *other_card) {
                (Card::J, Card::J) => {
                    continue;
                },
                (Card::J, _) => {
                    return Some(Ordering::Less)
                },
                (_, Card::J) => {
                    return Some(Ordering::Greater)
                },
                _ => {}
            }

            if own_card != other_card {
                return own_card.partial_cmp(other_card);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }

    fn kind_part1(&self) -> HandKind {
        if self.counts.len() == 1 {
            return HandKind::FiveOfAKind;
        }
        if self.counts.len() == 2 {
            let counts = self.counts.iter().collect::<Vec<_>>();
            let first = *counts.get(0).unwrap();

            if *first.1 == 1 || *first.1 == 4 {
                return HandKind::FourOfAKind;
            }

            if *first.1 == 2 || *first.1 == 3 {
                return HandKind::FullHouse;
            }
        }

        if self.counts.len() == 3 {
            let counts = self.counts.iter().collect::<Vec<_>>();
            if counts.iter().any(|(_, &c)| c == 3) {
                return HandKind::ThreeOfAKind;
            }
            return HandKind::TwoPair;
        }

        if self.counts.len() == 4 {
            return HandKind::OnePair;
        }
        HandKind::HighCard
    }
    
    fn kind_part2(&self) -> HandKind {
        let num_wildcards = *self.counts.get(&Card::J).unwrap_or(&0);
        if num_wildcards == 5 {
            return HandKind::FiveOfAKind;
        }
        return match self.counts.len() {
            1 => HandKind::FiveOfAKind,
            2 => {
                if num_wildcards > 0 {
                    return HandKind::FiveOfAKind;
                }
                
                let counts = self.counts.iter().collect::<Vec<_>>();
                let first = *counts.get(0).unwrap();

                if *first.1 == 1 || *first.1 == 4 {
                    HandKind::FourOfAKind
                } else {
                    HandKind::FullHouse
                }
            },
            3 => {
                if num_wildcards == 2 || num_wildcards == 3 {
                    return HandKind::FourOfAKind;
                }

                if num_wildcards == 0 {
                    let counts = self.counts.iter().collect::<Vec<_>>();
                    if counts.iter().any(|(_, &c)| c == 3) {
                        return HandKind::ThreeOfAKind;
                    }
                    return HandKind::TwoPair;
                }

                if self.counts.iter().any(|(_, &c)| c == 3) {
                    return HandKind::FourOfAKind;
                }
                if self.counts.iter().filter(|(_, &c)| c == 2).count() == 2 {
                    return HandKind::FullHouse;
                }
                if self.counts.iter().filter(|(_, &c)| c == 2).count() == 1 {
                    return HandKind::ThreeOfAKind;
                }
                HandKind::OnePair
            },
            4 => {
                if num_wildcards == 0 {
                    return HandKind::OnePair;
                }
                if num_wildcards == 1 {
                    return HandKind::ThreeOfAKind;
                }
                HandKind::ThreeOfAKind
            },
            5 => {
                if num_wildcards == 0 {
                    return HandKind::HighCard;
                }
                HandKind::OnePair
            }
            _ => unreachable!("welp. an uncovered case? unlikely: {}", self)
        }
    }
}

impl Hand {
    pub fn new(s: &str) -> Self {
        let (hand_str, bid) = s.split_once(" ").unwrap();
        let data: [Card; 5] = hand_str.chars().map(|c| c.try_into().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        let bid = bid.parse().unwrap();

        let mut counts: HashMap<Card, usize> = HashMap::default();

        for card in data.iter() {
            counts
            .entry(*card)
            .and_modify(|c| {*c += 1})
            .or_insert(1);
        }

        Self {
            cards: data,
            bid,
            counts
        }
    }


}


#[cfg(test)]
mod tests {

    #[test]
    fn test_smol_data() {
        let data = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(super::solve(data, |hand1, hand2| hand1.cmp_part1(hand2)), 6440);
        assert_eq!(super::solve(data, |hand1, hand2| hand1.cmp_part2(hand2)), 5905);
    }
}
