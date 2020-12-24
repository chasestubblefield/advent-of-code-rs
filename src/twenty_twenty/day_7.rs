use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let bc = BagContents::from_input(&input);
    bc.count_can_contain("shiny gold")
}

pub fn part2(input: &str) -> u32 {
    let bc = BagContents::from_input(&input);
    bc.count_contents("shiny gold")
}

#[derive(Debug)]
struct BagContents<'a> {
    map: HashMap<&'a str, Vec<(u32, &'a str)>>,
}

impl<'a> BagContents<'a> {
    fn from_input(input: &str) -> BagContents {
        let mut bag_contents = HashMap::new();
        for line in input.lines() {
            let (bag, contents) = parse_bag_and_contents(line);
            let contents = parse_contents(contents);
            bag_contents.insert(bag, contents);
        }
        BagContents { map: bag_contents }
    }

    fn count_can_contain(&self, color: &str) -> u32 {
        let mut count = 0;
        for &key in self.map.keys() {
            if self.can_contain(key, color) {
                count += 1;
            }
        }
        count
    }

    fn can_contain(&self, outer: &str, inner: &str) -> bool {
        let possible = self.map.get(outer).unwrap();
        if possible.is_empty() {
            false
        } else {
            possible
                .iter()
                .any(|(_num, color)| *color == inner || self.can_contain(color, inner))
        }
    }

    fn count_contents(&self, color: &str) -> u32 {
        self.map
            .get(color)
            .unwrap()
            .iter()
            .fold(0, |acc, (num, color)| {
                acc + num + num * self.count_contents(color)
            })
    }
}

fn parse_bag_and_contents(line: &str) -> (&str, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
}

fn parse_contents(contents: &str) -> Vec<(u32, &str)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (.*?) bags?").unwrap();
    }
    RE.captures_iter(contents)
        .map(|caps| {
            (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_7.txt";
    const SAMPLE: &str = "./input/2020/day_7_sample.txt";
    const PART1_SAMPLE: u32 = 4;
    const PART1: u32 = 179;
    const PART2_SAMPLE: u32 = 32;
    const PART2: u32 = 18925;

    use std::fs;

    #[test]
    fn part1_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART1_SAMPLE, super::part1(&input));
    }

    #[test]
    fn part1_full() {
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
