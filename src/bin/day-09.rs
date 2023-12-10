


pub type Sequence = std::collections::LinkedList<isize>;

pub fn main() {
    let data = include_str!("../../data/09.in");
    let sequences: Vec<Sequence> = data.lines().map(parse_sequence).collect();
    println!("part 1: {}", solve_part1(&sequences));
    println!("part 2: {}", solve_part2(&sequences));
    // assert!(submit(9, 1, &solve_part1(&sequences).to_string()).unwrap());
    // assert!(submit(9, 2, &solve_part2(&sequences).to_string()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::{parse_sequence, solve_part1, solve_part2, Sequence};

    #[test]
    fn test_smol_data() {
        let data = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sequences: Vec<Sequence> = data.lines().map(parse_sequence).collect();

        assert_eq!(solve_part1(&sequences), 114);
        assert_eq!(solve_part2(&sequences), 2);
    }
}

pub fn solve_part1(sequences: &[Sequence]) -> isize {
    sequences
        .iter()
        .map(|seq| predict_next(build_stack(seq)))
        .sum()
}

pub fn solve_part2(sequences: &[Sequence]) -> isize {
    sequences
        .iter()
        .map(|seq| predict_previous(build_stack(seq)))
        .sum()
}

pub fn predict_next(mut stack: Vec<Sequence>) -> isize {
    for index in (1..stack.len()).rev() {
        let to_add = *stack[index].back().unwrap();
        let to_predict_for = &mut stack[index - 1];
        to_predict_for.push_back(to_add + to_predict_for.back().unwrap());
    }
    *stack[0].back().unwrap()
}

pub fn predict_previous(mut stack: Vec<Sequence>) -> isize {
    for index in (1..stack.len()).rev() {
        let to_subtract = *stack[index].front().unwrap();
        let to_predict_for = &mut stack[index - 1];
        to_predict_for.push_front(to_predict_for.front().unwrap() - to_subtract);
    }
    *stack[0].front().unwrap()
}

pub fn parse_sequence(line: &str) -> Sequence {
    line.split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Sequence, _>>()
        .unwrap()
}

pub fn build_stack(sequence: &Sequence) -> Vec<Sequence> {
    let mut stack = vec![sequence.clone()];
    let mut diff = diffs(stack.last().unwrap());

    loop {
        if diff.iter().any(|&d| d != 0) {
            stack.push(diff.clone());
            diff = diffs(&diff);
        } else {
            break;
        }
    }
    stack
}

pub fn diffs(seq: &Sequence) -> Sequence {
    seq.iter()
        .skip(1)
        .zip(seq.iter().take(seq.len() - 1))
        .map(|(&next, &prev)| next - prev)
        .collect()
}
