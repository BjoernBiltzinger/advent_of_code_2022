use advent_of_code::helpers::sum_list;
use itertools::Itertools;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

pub fn part_one(input: &str) -> Option<u32> {
    let split = input.split("\n\n");
    let n = split.clone().count();
    let mut arr = vec![0; n]; //split.size_hint()];


    for (idx, s) in split.enumerate() {
        arr[idx] = sum_list(s);
    }

    arr.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.split("\n\n");
    let n = split.clone().count();
    let mut arr = vec![0; n]; //split.size_hint()];


    for (idx, s) in split.enumerate() {
        arr[idx] = sum_list(s);
    }

    Some(arr.iter().sorted().rev().take(3).sum())

    //arr.sort();

    //Some(arr[n-1]+arr[n-2]+arr[n-3])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
