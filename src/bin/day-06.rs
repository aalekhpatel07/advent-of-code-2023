pub fn main() {
    let data = include_str!("../../data/06.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_smol_data() {
        let data = r"Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(solve_part1(data), 288);
        assert_eq!(solve_part2(data), 71503);
    }


    #[test]
    fn test_big_data() {
        let data = r"Time:        46     68     98     66
Distance:   358   1054   1807   1080";

        assert_eq!(solve_part1(data), 138915);
        assert_eq!(solve_part2(data), 27340847);
    }

}

#[inline(always)]
pub fn count_wins(max_time: u64, min_distance: u64) -> usize {
    (0..max_time).filter(
        |hold_button_for| ((max_time - hold_button_for) * hold_button_for) > min_distance
    ).count()
}

pub fn solve_part1(data: &str) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let times = data.first().unwrap().strip_prefix("Time:").unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    let distances = data.last().unwrap().strip_prefix("Distance:").unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();

    times.iter().zip(distances.iter()).map(|(&max_time, &min_distance)| count_wins(max_time, min_distance)).product()
}

pub fn solve_part2(data: &str) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let max_time: u64 = data.first().unwrap().strip_prefix("Time:").unwrap().chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
    let min_distance: u64 = data.last().unwrap().strip_prefix("Distance:").unwrap().chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

    count_wins(max_time, min_distance)
}

