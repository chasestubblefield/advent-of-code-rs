use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines()
        .map(|l| l.parse().expect("Parse error"))
        .collect();

    let (x1, x2) = find_two(&nums, 2020).unwrap();
    x1 * x2
}

pub fn part_2(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines()
    .map(|l| l.parse().expect("Parse error"))
    .collect();

    let (x1, x2, x3) = find_three(&nums, 2020).unwrap();
    x1 * x2 * x3
}

fn find_two(nums: &[u32], sum: u32) -> Option<(u32, u32)> {
    let mut seen = HashSet::new();

    for &x in nums {
        if x > sum {
            continue;
        }

        let target = sum - x;

        if seen.contains(&target) {
            return Some((x, target));
        }

        seen.insert(x);
    }

    None
}

fn find_three(nums: &[u32], sum: u32) -> Option<(u32, u32, u32)> {
    for &x1 in nums {
        if let Some((x2, x3)) = find_two(nums, sum - x1) {
            return Some((x1, x2, x3));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_1.txt";
    const SAMPLE: &str = "./input/2020/day_1_sample.txt";
    const PART_1_SAMPLE: u32 = 514579;
    const PART_1: u32 = 858496;
    const PART_2_SAMPLE: u32 = 241861950;
    const PART_2: u32 = 263819430;

    use std::fs;

    #[test]
    fn part_1_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_1_SAMPLE, super::part_1(&input));
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_1, super::part_1(&input));
    }

    #[test]
    fn part_2_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_2_SAMPLE, super::part_2(&input));
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_2, super::part_2(&input));
    }
}
