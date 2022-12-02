use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SCORE: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("A X", 4);
        map.insert("B X", 1);
        map.insert("C X", 7);
        map.insert("A Y", 8);
        map.insert("B Y", 5);
        map.insert("C Y", 2);
        map.insert("A Z", 3);
        map.insert("B Z", 9);
        map.insert("C Z", 6);
        map.insert("", 0);
        map
    };
}

pub fn part_one(input: &str) -> Option<u32> {

    input.split("\n").map(|g| SCORE.get(g)).sum()

}

lazy_static! {
    static ref SCORE2: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("A X", 3);
        map.insert("B X", 1);
        map.insert("C X", 2);
        map.insert("A Y", 4);
        map.insert("B Y", 5);
        map.insert("C Y", 6);
        map.insert("A Z", 8);
        map.insert("B Z", 9);
        map.insert("C Z", 7);
        map.insert("", 0);
        map
    };
}

pub fn part_two(input: &str) -> Option<u32> {

    input.split("\n").map(|g| SCORE2.get(g)).sum()

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
