/// Given two numbers, compute their greatest common divisor,
/// i.e. the largest number that divides both of the given numbers evenly,
pub fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
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
pub fn lcm_many<'a>(numbers: impl Iterator<Item = &'a usize>) -> Option<usize> {
    numbers.copied().reduce(lcm)
}

/// Given an initial state of a finite state machine,
/// and a transition function between states, detect
/// the smallest cycle in the state chain. If found,
/// return the number of state transitions required to
/// get to the cycle, along with the length of the cycle.
///
/// Optionally, we can also limit the search for cycle
/// to a maximum number of steps to guarantee eventual
/// termination.
///
/// A cycle may look like:
/// ```s0, s1, s2, ..., s5, (s6, s7, s8)*```
///
/// In that case, this function would return ```(6, 3)```
pub fn detect_cycle<S, F>(
    initial_state: &S,
    transition_fn: F,
    max_steps: Option<usize>,
) -> Option<(usize, usize)>
where
    F: Fn(&S) -> S,
    S: Clone + std::hash::Hash + PartialEq + Eq,
{
    let mut step = 0;
    let mut first_occurrences = std::collections::HashMap::<S, usize>::new();

    #[allow(unused_assignments)]
    let mut repeating_key = None;
    let mut state = initial_state.clone();

    loop {
        if let Some(max_steps) = max_steps {
            if step > max_steps {
                return None;
            }
        }

        if first_occurrences.contains_key(&state) {
            repeating_key = Some((state, step));
            break;
        }
        first_occurrences.insert(state.clone(), step);
        state = transition_fn(&state);
        step += 1;
    }

    let (repeating_key, second_occurrence) = repeating_key.unwrap();
    let first_occurrence = *first_occurrences.get(&repeating_key).unwrap();

    Some((first_occurrence, (second_occurrence - first_occurrence)))
}
