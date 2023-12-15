use regex::Regex;

fn part1_hash(s: &str) -> usize {
    let mut current = 0;
    for c in s.chars().filter(|c| *c != '\n') {
        let code = (c as u8) as usize;
        current = ((current + code) * 17) % 256;
    }
    current
}

fn solve_part2(data: &str) -> usize {
    const VAL: Vec<(String, usize)> = vec![];
    let mut boxes: [Vec<(String, usize)>; 256] = [VAL; 256];

    let pattern_re: Regex = Regex::new(r"(\w+)([-=])(\d)?").unwrap();

    data.split(',').for_each(|ins| {
        let caps = pattern_re.captures(ins).unwrap();

        let lens = caps.get(1).unwrap().as_str();
        let sign = caps.get(2).unwrap().as_str();

        let box_index = part1_hash(lens);

        let valid_box = &mut boxes[box_index];
        if sign == "=" {
            let power: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            if let Some(found_index) = valid_box
                .iter()
                .position(|(lens_in_box, _)| lens_in_box == &lens)
            {
                valid_box[found_index] = (lens.to_string(), power);
            } else {
                valid_box.push((lens.to_string(), power));
            }
        } else {
            valid_box.retain(|(lens_in_box, _)| lens_in_box != &lens);
        }
    });

    (0..256usize)
        .map(|box_idx| {
            boxes[box_idx]
                .iter()
                .enumerate()
                .map(|(slot, (_, power))| (slot + 1) * *power * (box_idx + 1))
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test_sample() {
    let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(solve_part1(data), 1320);
    assert_eq!(solve_part2(data), 145);
}

fn solve_part1(data: &str) -> usize {
    data.split(',').map(part1_hash).sum()
}

fn main() {
    let data = include_str!("../../data/15.in");
    println!("Part 1: {}", solve_part1(data));
    println!("Part 2: {}", solve_part2(data));
}
