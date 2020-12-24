use std::collections::BTreeSet;

pub fn part1(input: &str) -> u32 {
    let ids: BTreeSet<u32> = input.lines().map(|l| seat_id(l)).collect();
    ids.into_iter().next_back().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let ids: BTreeSet<u32> = input.lines().map(|l| seat_id(l)).collect();
    let mut counter = *ids.iter().next().unwrap();
    for &id in ids.iter() {
        if id != counter {
            return counter;
        }
        counter += 1;
    }
    panic!("No possible seat ID found!");
}

fn seat_id(s: &str) -> u32 {
    let row_binary = s[0..7].replace("F", "0").replace("B", "1");
    let col_binary = s[7..10].replace("L", "0").replace("R", "1");

    let row_num = u32::from_str_radix(&row_binary, 2).unwrap();
    let col_num = u32::from_str_radix(&col_binary, 2).unwrap();

    row_num * 8 + col_num
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_5.txt";
    const PART1: u32 = 883;
    const PART2: u32 = 532;

    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART1, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART2, super::part2(&input));
    }
}
