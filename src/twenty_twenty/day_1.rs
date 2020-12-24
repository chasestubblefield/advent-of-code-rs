use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines()
        .map(|l| l.parse().expect("Parse error"))
        .collect();

    let (x1, x2) = find_two(&nums, 2020).unwrap();
    x1 * x2
}

pub fn part2(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines()
    .map(|l| l.parse().expect("Parse error"))
    .collect();

    let (x1, x2, x3) = find_three(&nums, 2020).unwrap();
    x1 * x2 * x3
}

fn find_two(nums: &Vec<u32>, sum: u32) -> Option<(u32, u32)> {
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

fn find_three(nums: &Vec<u32>, sum: u32) -> Option<(u32, u32, u32)> {
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
    const PART1_SAMPLE: u32 = 514579;
    const PART1: u32 = 858496;
    const PART2_SAMPLE: u32 = 241861950;
    const PART2: u32 = 263819430;

    use std::fs;

    #[test]
    fn part1_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART1_SAMPLE, super::part1(&input));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART1, super::part1(&input));
    }

    #[test]
    fn part2_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART2_SAMPLE, super::part2(&input));
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART2, super::part2(&input));
    }
}
