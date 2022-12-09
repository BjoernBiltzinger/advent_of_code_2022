// first simple solution - simple search algorithm

pub fn run_simple(input: &str, len_string: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();

    let mut found;

    for start_id in 0..chars.len(){
        found = true;
        for n in 0..len_string{
            for m in n+1..len_string{
                if chars[start_id+n]==chars[start_id+m] {
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
    unreachable!()
}

pub fn part_one_simple(input: &str) -> Option<usize> {
    run_simple(input, 4)
}

pub fn part_two_simple(input: &str) -> Option<usize> {
    run_simple(input, 14)
}

// Optimized version with a advanced search algorithm

// using a char vector, loop over the indicies and get the values from the vec is
// faster than using sliced strings.

// makes the search for the long string much faster (factor ~10) compared
// to the simple approach
pub fn run(input: &str, len_string: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();

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
    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
   run(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
   run(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6, None);

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
        let input = advent_of_code::read_file("examples", 6, None);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6, None);
        assert_eq!(part_two(&input), Some(19));
    }
}
