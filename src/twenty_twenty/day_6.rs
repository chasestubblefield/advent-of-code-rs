use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    input.split("\n\n").fold(0, |acc, group| {
        let set: HashSet<char> = group.replace("\n", "").chars().collect();
        acc + (set.len() as u32)
    })
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    for group in input.split("\n\n") {
        let mut answers = HashMap::new();
        for line in group.lines() {
            for c in line.chars() {
                *answers.entry(c).or_insert(0) += 1;
            }
        }

        let num_people = group.lines().count();
        for (_q, score) in answers {
            if score == num_people {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_6.txt";
    const SAMPLE: &str = "./input/2020/day_6_sample.txt";
    const PART1_SAMPLE: u32 = 11;
    const PART1: u32 = 6534;
    const PART2_SAMPLE: u32 = 6;
    const PART2: u32 = 3402;

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
