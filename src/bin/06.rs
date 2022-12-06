// first simple solution - not very fast, especially part 2

pub fn part_one_simple(input: &str) -> Option<usize> {
    let length = input.len();
    let mut sub_slice: String;
    let mut found: bool;
    for i in 0..length-4 {
        found = true;
        sub_slice = input[i..i+4].to_string();
        for (n, c1) in sub_slice.chars().enumerate(){
            for c2 in sub_slice.chars().skip(n+1){
                if c1 == c2 {
                    found = false;
                    break;
                }
            }
            if !found{
                break
            }
        }
        if found {
            return Some(i+4);
        }
    }
    None
}

pub fn part_two_simple(input: &str) -> Option<usize> {
    let length = input.len();
    let mut sub_slice: String;
    let mut found: bool;
    for i in 0..length-14 {
        found = true;
        sub_slice = input[i..i+14].to_string();
        for (n, c1) in sub_slice.chars().enumerate(){
            for c2 in sub_slice.chars().skip(n+1){
                if c1 == c2 {
                    found = false;
                    break;
                }
            }
            if !found{
                break
            }
        }
        if found {
            return Some(i+14);
        }
    }
    None
}

// Optimized version with a smarter search algorithm

pub fn run(input: &str, len_string: usize) -> Option<usize> {
    let mut sub_slice: String;

    let mut start_id = 0;
    let mut found = false;

    while !found {
        found = true;
        sub_slice = input[start_id..start_id+len_string].to_string();
        for (n, c1) in sub_slice.chars().rev().skip(1).enumerate(){
            for c2 in sub_slice.chars().skip(len_string-1-n){
                if c1==c2 {
                    start_id += len_string-1-n;
                    found = false;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        if found {
            return Some(start_id+len_string);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
   run(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
   run(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);

    advent_of_code::solve!("1 simple", part_one_simple, input);
    advent_of_code::solve!("2 simple", part_two_simple, input);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
