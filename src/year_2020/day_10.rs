pub fn part_1(input: &str) -> u32 {
    let mut adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    adapters.push(adapters.last().unwrap() + 3);

    let mut one_count = 0;
    let mut three_count = 0;
    let mut prev = 0;
    for a in adapters.iter() {
        match a - prev {
            1 => {
                one_count += 1;
            }
            2 => {}
            3 => {
                three_count += 1;
            }
            _ => {
                panic!("Missing adapter!")
            }
        }
        prev = *a;
    }
    one_count * three_count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_10.txt";
    const SAMPLE_1: &str = "./input/2020/day_10_sample_1.txt";
    const SAMPLE_2: &str = "./input/2020/day_10_sample_2.txt";
    const PART_1_SAMPLE_1: u32 = 35;
    const PART_1_SAMPLE_2: u32 = 220;
    const PART_1: u32 = 1848;
    // const PART_2_SAMPLE: i32 = 8;
    // const PART_2: i32 = 1160;

    use std::fs;

    #[test]
    fn part_1_sample_1() {
        let input = fs::read_to_string(SAMPLE_1).unwrap();
        assert_eq!(PART_1_SAMPLE_1, super::part_1(&input));
    }

    #[test]
    fn part_1_sample_2() {
        let input = fs::read_to_string(SAMPLE_2).unwrap();
        assert_eq!(PART_1_SAMPLE_2, super::part_1(&input));
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_1, super::part_1(&input));
    }

    // #[test]
    // fn part_2_sample() {
    //     let input = fs::read_to_string(SAMPLE).unwrap();
    //     assert_eq!(PART_2_SAMPLE, super::part_2(&input));
    // }

    // #[test]
    // fn part_2() {
    //     let input = fs::read_to_string(INPUT).unwrap();
    //     assert_eq!(PART_2, super::part_2(&input));
    // }
}
