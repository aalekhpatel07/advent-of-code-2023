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

/// Find out the number of integer points that satisfy:
///
/// (max_time - x) * x > min_distance
/// where x in [0..max_time]
///
/// Straightforward approach of finding the interval with the quadratic
/// formula, then counting the integers that lie inside (and not including)
/// the interval.
#[inline(always)]
pub fn count_wins(max_time: u64, min_distance: u64) -> usize {
    let mut lower_bound =
        0.5 * (max_time as f64 - ((max_time as f64).powf(2.0) - 4.0 * min_distance as f64).sqrt());
    let mut upper_bound =
        0.5 * (max_time as f64 + ((max_time as f64).powf(2.0) - 4.0 * min_distance as f64).sqrt());

    // If the boundary happens to be an integer,
    // we need to exclude that from the count.
    if lower_bound.fract() == 0.0 {
        lower_bound += 1.0;
    } else {
        lower_bound = lower_bound.ceil();
    }

    if upper_bound.fract() == 0.0 {
        upper_bound -= 1.0;
    } else {
        upper_bound = upper_bound.floor();
    }

    upper_bound as usize - lower_bound as usize + 1
}

pub fn solve_part1(data: &str) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let times = data
        .first()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let distances = data
        .last()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(&max_time, &min_distance)| count_wins(max_time, min_distance))
        .product()
}

pub fn solve_part2(data: &str) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let max_time: u64 = data
        .first()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let min_distance: u64 = data
        .last()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    count_wins(max_time, min_distance)
}
