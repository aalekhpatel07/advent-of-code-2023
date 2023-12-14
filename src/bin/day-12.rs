use rayon::prelude::*;


pub fn main() {
    let data = include_str!("../../data/12.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(data: &str) -> usize {
    
    data
    .lines()
    .par_bridge()
    .map(|line|{
        let (seq, groups) = line.split_once(" ").unwrap();
        Arrangement {
            seq: seq.chars().map(|c| c as u8).collect(),
            groups: groups.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    })
    .map(|arrangement| arrangement.count_by_skipping_contiguous_blocks())
    .sum()
}
pub fn solve_part2(data: &str) -> usize {
    let res = data
    .lines()
    .par_bridge()
    .map(|line|{
        let (seq, groups) = line.split_once(" ").unwrap();

        let new_seq = vec![seq.to_string(); 5].join("?");
        let new_groups = vec![groups.to_string(); 5].join(",");

        Arrangement {
            seq: new_seq.chars().map(|c| c as u8).collect(),
            groups: new_groups.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    })
    .map(|arrangement| { 
        arrangement.count_by_skipping_contiguous_blocks()
    })
    .sum();
    res
}

#[derive(Debug, Clone)]
pub struct Arrangement {
    pub seq: Vec<u8>,
    pub groups: Vec<usize>
}

impl Arrangement {

    pub fn count_by_skipping_contiguous_blocks(
        &self,
    ) -> usize {
        let mut cache = std::collections::HashMap::new();
        Self::count_valid_arrangments(&self.seq, &self.groups, &mut cache)
    }

    pub fn count_valid_arrangments(arrangement: &[u8], contiguous_blocks: &[usize], cache: &mut std::collections::HashMap<(Vec<u8>, Vec<usize>), usize>) -> usize {
        let key = (arrangement.to_vec(), contiguous_blocks.to_vec());
        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }

        // Reached end of string, so only if all contiguous broken blocks
        // have been used (or charted) should we count this as a valid arrangement.
        if arrangement.is_empty() {
            return match contiguous_blocks.is_empty() {
                true => 1,
                false => 0
            }
        }

        // No more contiguous broken blocks left,
        // so whatever remains must be non-'#' to be counted 
        // as a valid arrangement.
        if contiguous_blocks.is_empty() {
            return match arrangement.iter().any(|&c| c == b'#') {
                true => 0,
                false => 1
            }
        }

        let mut result = 0;

        // A valid arrangement either requires wildcards to be a working spring,
        // in which case, the number of valid arrangments is the same as those of 
        // starting at the next spring.
        if arrangement[0] == b'.' || arrangement[0] == b'?' {
            result += Self::count_valid_arrangments(&arrangement[1..], contiguous_blocks, cache);
        }

        // Or a valid arrangement requires wildcard to be a broken spring.
        // We're at the start of a broken block.
        if arrangement[0] == b'#' || arrangement[0] == b'?' {   

            let damaged_springs_in_this_block = contiguous_blocks[0];
            let springs_left = arrangement.len();
            let have_enough_springs = springs_left >= damaged_springs_in_this_block;
            let block_contains_only_damaged_strings = arrangement[..damaged_springs_in_this_block.min(arrangement.len())].iter().all(|&c| c == b'#' || c == b'?');

            if have_enough_springs && block_contains_only_damaged_strings // since the broken spring block is contiguous.
            && (
                // Either exactly these many springs left, all of which are broken.
                (damaged_springs_in_this_block == springs_left) 

                // or the spring immediately following this block isn't a broken spring, 
                // as otherwise it'd have to be a part of this block itself.
                || arrangement[damaged_springs_in_this_block] != b'#' 
            ) {
                // The spring following this block is necessarily a '.' or a '?' but 
                // even if it is the wildcard, it'll still have to be treated as a working spring
                // because we're jumping all the way to the start of the next block of broken springs, if any.
                if damaged_springs_in_this_block + 1 <= springs_left {
                    result += Self::count_valid_arrangments(&arrangement[damaged_springs_in_this_block + 1..], &contiguous_blocks[1..], cache);
                }
                // Ending this block brings us to the end of the spring arrangement.
                else {
                    result += Self::count_valid_arrangments(&[], &contiguous_blocks[1..], cache);
                }
            }
        }
        cache.insert(key, result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn expand() {
        let data = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(solve_part1(data), 21);
        assert_eq!(solve_part2(data), 525152);
    }
}
