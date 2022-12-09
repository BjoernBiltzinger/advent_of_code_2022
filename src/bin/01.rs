use advent_of_code::helpers::sum_list;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let split = input.split("\n\n");
    let n = split.clone().count();
    let mut arr = vec![0; n];


    for (idx, s) in split.enumerate() {
        arr[idx] = sum_list(s);
    }

    arr.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.split("\n\n");
    let n = split.clone().count();
    let mut arr = vec![0; n];


    for (idx, s) in split.enumerate() {
        arr[idx] = sum_list(s);
    }

    Some(arr.iter().sorted().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::read_file("examples", 1, None);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::read_file("examples", 1, None);
        assert_eq!(part_two(&input), Some(45000));
    }
}
