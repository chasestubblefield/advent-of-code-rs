use array2d::Array2D;

pub fn part_1(input: &str) -> u32 {
    let grid = grid_from_input(input);
    count_slope(&grid, 3, 1)
}

pub fn part_2(input: &str) -> u32 {
    let grid = grid_from_input(input);

    let count1 = count_slope(&grid, 1, 1);
    let count2 = count_slope(&grid, 3, 1);
    let count3 = count_slope(&grid, 5, 1);
    let count4 = count_slope(&grid, 7, 1);
    let count5 = count_slope(&grid, 1, 2);

    count1 * count2 * count3 * count4 * count5
}

fn grid_from_input(input: &str) -> Array2D<char> {
    let mut grid = vec![];
    for line in input.lines() {
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }
    Array2D::from_rows(&grid)
}

fn count_slope(grid: &Array2D<char>, right: usize, down: usize) -> u32 {
    let mut row = 0;
    let mut col = 0;
    let mut count = 0;

    while row < grid.num_rows() {
        if grid[(row, col)] == '#' {
            count += 1;
        }

        row += down;
        col = (col + right) % grid.num_columns();
    }

    count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_3.txt";
    const SAMPLE: &str = "./input/2020/day_3_sample.txt";
    const PART_1_SAMPLE: u32 = 7;
    const PART_1: u32 = 254;
    const PART_2_SAMPLE: u32 = 336;
    const PART_2: u32 = 1666768320;

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
