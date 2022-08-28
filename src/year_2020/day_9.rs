use std::collections::{HashSet, VecDeque};

pub fn part_1(input: &str, preamble: usize) -> u64 {
    let list: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut window: VecDeque<&u64> = list.iter().take(preamble).collect();
    let mut i = preamble;
    while i < list.len() {
        let num = &list[i];
        if !find_two(&window, num) {
            return *num;
        }
        i += 1;
        window.pop_front();
        window.push_back(num);
    }
    panic!("Could not find invalid number!");
}

pub fn part_2(input: &str, target: u64) -> u64 {
    let list: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    for window_size in 2..list.len() {
        let mut iter = list.windows(window_size);
        let mut window = iter.next().unwrap();
        let mut sum: u64 = window.iter().sum();
        loop {
            if sum == target {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }

            sum -= window.first().unwrap();
            if let Some(next_window) = iter.next() {
                window = next_window;
            } else {
                break;
            }
            sum += window.last().unwrap();
        }
    }
    panic!("Could not find!");
}

fn find_two(window: &VecDeque<&u64>, sum: &u64) -> bool {
    let mut seen = HashSet::new();
    for &x in window.iter() {
        if x > sum {
            continue;
        }
        let target = sum - x;

        if seen.contains(&target) {
            return true;
        }

        seen.insert(x);
    }
    false
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_9.txt";
    const SAMPLE: &str = "./input/2020/day_9_sample.txt";
    const PART_1_SAMPLE: u64 = 127;
    const PART_1: u64 = 15690279;
    const PART_2_SAMPLE: u64 = 62;
    const PART_2: u64 = 2174232;

    use std::fs;

    #[test]
    fn part_1_sample() {
        let path = module_path!();
        println!("{:?}", path);
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_1_SAMPLE, super::part_1(&input, 5));
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_1, super::part_1(&input, 25));
    }

    #[test]
    fn part_2_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_2_SAMPLE, super::part_2(&input, PART_1_SAMPLE));
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_2, super::part_2(&input, PART_1));
    }
}
