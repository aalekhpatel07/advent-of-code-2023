pub mod part1 {
    /// Given a string with digits, find the first and the last
    /// occurrence of a digit and combine them to form a two digit
    /// number.
    pub fn parse_digits(s: &str) -> u32 {
        let mut first_digit = None;
        let mut last_digit = None;

        for char in s.chars() {
            if char.is_ascii_digit() {
                // Only record the first digit if not already recorded.
                if first_digit.is_none() {
                    first_digit = Some(char.to_digit(10).unwrap());
                }
                last_digit = Some(char.to_digit(10).unwrap());
            }
        }

        first_digit.unwrap() * 10 + last_digit.unwrap()
    }
}

pub mod part2 {
    /// Given a string with digits or the English word
    /// representation of the digits, find the first
    /// and the last occurrence of a digit and combine
    /// them to form a two digit number.
    pub fn parse_digits(s: &str) -> u32 {
        let mut first_digit = None;
        let mut last_digit = None;

        let digits = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (idx, char) in s.chars().enumerate() {
            // Either a digit
            if char.is_ascii_digit() {
                // Only record the first digit if not already recorded.
                if first_digit.is_none() {
                    first_digit = Some(char.to_digit(10).unwrap());
                }
                last_digit = Some(char.to_digit(10).unwrap());
            } else {
                // or an English word.
                for (digit_idx, digit) in digits.iter().enumerate() {
                    if s[idx..].starts_with(digit) {
                        // Only record the first digit if not already recorded.
                        if first_digit.is_none() {
                            first_digit = Some(digit_idx as u32 + 1);
                        }
                        last_digit = Some(digit_idx as u32 + 1);
                        // we can only have at most one digit spelling out at this index,
                        // so no need to check other digits.
                        break;
                    }
                }
            }
        }

        first_digit.unwrap() * 10 + last_digit.unwrap()
    }
}

fn solve_part1(data: &str) {
    let mut sum = 0;
    for line in data.lines() {
        let line_res = part1::parse_digits(line);
        // eprintln!("res: {line_res}, line: {line}");
        sum += line_res
    }
    println!("part 1: {sum}")
}

fn solve_part2(data: &str) {
    let mut sum = 0;
    for line in data.lines() {
        let line_res = part2::parse_digits(line);
        // eprintln!("res: {line_res}, line: {line}");
        sum += line_res
    }
    println!("part 2: {sum}")
}

pub fn main() {
    let data = include_str!("../../data/01.in");
    solve_part1(data);
    solve_part2(data);
}
