use regex::Regex;

pub fn part_1(input: &str) -> usize {
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

pub fn part_2(input: &str) -> usize {
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
    const PART_1_SAMPLE: usize = 2;
    const PART_1: usize = 645;
    const PART_2_SAMPLE: usize = 1;
    const PART_2: usize = 737;

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
