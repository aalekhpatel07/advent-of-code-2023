
/// Given two numbers, compute their greatest common divisor,
/// i.e. the largest number that divides both of the given numbers evenly,
pub fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b
    }
    if b == 0 {
        return a
    }
    match a.cmp(&b) {
        std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => gcd(b, a % b),
        std::cmp::Ordering::Less => gcd(a, b % a),
    }
}

/// Given two numbers, compute their least common multiple,
/// i.e. the smallest number that is divisible by both of the given numbers.
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

/// Given a stream of numbers, compute the smallest number that is divisible by
/// all of the numbers in the stream.
pub fn lcm_many<'a>(numbers: impl Iterator<Item=&'a usize>) -> Option<usize> {
    numbers.copied()
    .reduce(lcm)
}