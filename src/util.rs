use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;

pub fn find_two_with_sum<'a, T: PartialOrd + Sub<Output = T> + Hash + Eq + Copy>(
    list: &'a [T],
    sum: &T,
) -> Option<(&'a T, &'a T)> {
    let mut seen = HashSet::new();

    for x in list.iter() {
        if x > sum {
            continue;
        }

        let target = *sum - *x;
        if seen.contains(&target) {
            return Some((x, &target));
        }

        seen.insert(x);
    }
    None
}

#[cfg(test)]
pub mod testing {
    pub fn get_path() -> &'static str {
        let mod_path = module_path!();
        println!("{}", mod_path);
        "./input/2020/day_9.txt"
    }
}
