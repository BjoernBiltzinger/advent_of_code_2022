use itertools::Itertools;

type Input<'a> = Vec<char>;

// first simple solution - simple search algorithm

pub fn run_simple(input: &str, len_string: usize) -> Option<usize> {
    input.chars().collect::<Input>()
        .windows(len_string)
        .find_position(|cs| {
            for n in 0..len_string{
                for m in n+1..len_string{
                    if cs[n]==cs[m]{
                        return false;
                     }
                }
            }
            true
        })
        .map(|(pos, _)| pos + len_string)
}

pub fn part_one_simple(input: &str) -> Option<usize> {
    run_simple(input, 4)
}

pub fn part_two_simple(input: &str) -> Option<usize> {
    run_simple(input, 14)
}

// Optimized version with a advanced search algorithm

// this solution with the  advanced search algorithm
// but with sub strings is also not optimal
pub fn run_with_string(input: &str, len_string: usize) -> Option<usize> {
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

// using a char vector, loop over the indicies and get the values from the vec seems to be faster
// makes the search for the long string much faster (factor ~7) compared to the simple approach
// Always use chars instead of strings if possible!
pub fn run(input: &str, len_string: usize) -> Option<usize> {
    let chars: Input = input.chars().collect();

    let mut start_id = 0;
    let mut found = false;

    while !found {
        found = true;
        for n in (0..len_string-1).rev(){
            for m in n+1..len_string{
                if chars[start_id+n]==chars[start_id+m] {
                    start_id += n+1;
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

    advent_of_code::solve!("1 simple search algorithm", part_one_simple, input);
    advent_of_code::solve!("2 simple search algorithm", part_two_simple, input);

    advent_of_code::solve!("1 advanced search algorithm", part_one, input);
    advent_of_code::solve!("2 advanced search algorithm", part_two, input);
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
