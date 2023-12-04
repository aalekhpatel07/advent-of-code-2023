
pub fn main() {
    let data = include_str!("../../data/04.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn get_wins_per_card(cards: &str) -> impl Iterator<Item=u32> + '_ {
    cards
    .lines()
    .map(|card| {
        let (_, rest) = card.split_once(": ").unwrap();
        let (winning_numbers, our_numbers) = rest.split_once(" | ").unwrap();

        let winning_numbers: std::collections::HashSet<String> = 
            winning_numbers
            .split_whitespace()
            .map(String::from)
            .collect();

        let our_numbers: std::collections::HashSet<String> = 
            our_numbers
            .split_whitespace()
            .map(String::from)
            .collect();

        winning_numbers.intersection(&our_numbers).count() as u32
    })
}

pub fn solve_part1(cards: &str) -> i32 {
    get_wins_per_card(cards)
    .filter_map(|v| {
        match v {
            0 => None,
            v => Some(2_i32.pow(v - 1_u32))
        }
    })
    .sum()
}

pub fn solve_part2(cards_str: &str) -> i32 {
    let cards = get_wins_per_card(cards_str).collect::<Vec<_>>();
    let mut frequencies: Vec<_> = vec![1; cards.len()];

    for (current_card, value) in cards.iter().enumerate() {
        for next_card in (current_card + 1)..(current_card + 1 + *value as usize) {
            frequencies[next_card] += frequencies[current_card];
        }
    }

    frequencies.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_smol_data() {
        let data = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(13, solve_part1(data));
        assert_eq!(30, solve_part2(data));
    }
}
