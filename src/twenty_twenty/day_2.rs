use regex::Regex;

pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    input.lines().filter(|line| {
        let caps = re.captures(line).unwrap();

        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let letter = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        let count = password.matches(letter).count();

        count >= min && count <= max
    }).count()
}

pub fn part2(input: &str) -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    input.lines().filter(|line| {
        let caps = re.captures(line).unwrap();

        let i1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let i2: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let letter = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        let c1 = &password[i1 - 1..i1];
        let c2 = &password[i2 - 1..i2];

        (c1 == letter) && (c2 != letter) || (c1 != letter) && (c2 == letter)
    }).count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_2.txt";
    const SAMPLE: &str = "./input/2020/day_2_sample.txt";
    const PART1_SAMPLE: usize = 2;
    const PART1: usize = 645;
    const PART2_SAMPLE: usize = 1;
    const PART2: usize = 737;

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
