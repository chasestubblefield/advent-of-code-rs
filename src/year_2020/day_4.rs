use regex::Regex;

pub fn part_1(input: &str) -> u32 {
    let byr = Regex::new(r"byr:\S+").unwrap();
    let iyr = Regex::new(r"iyr:\S+").unwrap();
    let eyr = Regex::new(r"eyr:\S+").unwrap();
    let hgt = Regex::new(r"hgt:\S+").unwrap();
    let hcl = Regex::new(r"hcl:\S+").unwrap();
    let ecl = Regex::new(r"ecl:\S+").unwrap();
    let pid = Regex::new(r"pid:\S+").unwrap();

    let mut count = 0;
    for passport in input.split("\n\n") {
        if byr.is_match(passport)
            && iyr.is_match(passport)
            && eyr.is_match(passport)
            && hgt.is_match(passport)
            && hcl.is_match(passport)
            && ecl.is_match(passport)
            && pid.is_match(passport)
        {
            count += 1;
        }
    }
    count
}

pub fn part_2(input: &str) -> u32 {
    let byr = Regex::new(r"\bbyr:(\d{4})\b").unwrap();
    let iyr = Regex::new(r"\biyr:(\d{4})\b").unwrap();
    let eyr = Regex::new(r"\beyr:(\d{4})\b").unwrap();
    let hgt = Regex::new(r"\bhgt:(\d{2,3})(cm|in)\b").unwrap();
    let hcl = Regex::new(r"\bhcl:#[0-9a-f]{6}\b").unwrap();
    let ecl = Regex::new(r"\becl:(?:amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid = Regex::new(r"\bpid:\d{9}\b").unwrap();

    let mut count = 0;
    for passport in input.split("\n\n") {
        if is_match_in_range(passport, &byr, 1920, 2002)
            && is_match_in_range(passport, &iyr, 2010, 2020)
            && is_match_in_range(passport, &eyr, 2020, 2030)
            && is_valid_height(passport, &hgt)
            && hcl.is_match(passport)
            && ecl.is_match(passport)
            && pid.is_match(passport)
        {
            count += 1;
        }
    }
    count
}

fn is_match_in_range(s: &str, re: &Regex, min: u32, max: u32) -> bool {
    match re.captures(s) {
        Some(caps) => {
            let num: u32 = caps[1].parse().unwrap();
            min <= num && num <= max
        }
        None => false,
    }
}

fn is_valid_height(s: &str, re: &Regex) -> bool {
    match re.captures(s) {
        Some(caps) => {
            let num: usize = caps[1].parse().unwrap();
            let unit = &caps[2];
            match unit {
                "cm" => (150..=193).contains(&num),
                "in" => (59..=76).contains(&num),
                _ => false,
            }
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_4.txt";
    const SAMPLE: &str = "./input/2020/day_4_sample.txt";
    const PART_1_SAMPLE: u32 = 2;
    const PART_1: u32 = 213;
    const PART_2_SAMPLE: u32 = 2;
    const PART_2: u32 = 147;

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
        assert_eq!(PART_2_SAMPLE, super::part_1(&input));
        let input = fs::read_to_string("./input/2020/day_4_part_2_invalid.txt").unwrap();
        assert_eq!(0, super::part_2(&input));
        let input = fs::read_to_string("./input/2020/day_4_part_2_valid.txt").unwrap();
        assert_eq!(4, super::part_2(&input));
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_2, super::part_2(&input));
    }
}
