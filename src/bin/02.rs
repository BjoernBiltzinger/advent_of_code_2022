use lazy_static::lazy_static;
use std::collections::HashMap;

// kind of clean but i think the hashmaps are slow due to the hashing
// but there are no unhashed maps

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

pub fn part_one_clean_but_slow(input: &str) -> Option<u32> {
    input.split("\n").map(|g| SCORE.get(g)).sum()
}

pub fn part_two_clean_but_slow(input: &str) -> Option<u32> {

    input.split("\n").map(|g| SCORE2.get(g)).sum()

}

// faster

pub fn lookup1(x: &str) -> u32{

    if x=="A X"{
        return 4;
    }
    if x=="B X"{
        return 1;
    }
    if x=="C X"{
        return 7;
    }
    if x=="A Y"{
        return 8;
    }
    if x=="B Y"{
        return 5;
    }
    if x=="C Y"{
        return 2;
    }
    if x=="A Z"{
        return 3;
    }
    if x=="B Z"{
        return 9;
    }
    if x=="C Z"{
        return 6;
    }
    return 0;
}

pub fn lookup2(x: &str) -> u32{

    if x=="A X"{
        return 3;
    }
    if x=="B X"{
        return 1;
    }
    if x=="C X"{
        return 2;
    }
    if x=="A Y"{
        return 4;
    }
    if x=="B Y"{
        return 5;
    }
    if x=="C Y"{
        return 6;
    }
    if x=="A Z"{
        return 8;
    }
    if x=="B Z"{
        return 9;
    }
    if x=="C Z"{
        return 7;
    }
    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n").map(|g| lookup1(g)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {

    Some(input.split("\n").map(|g| lookup2(g)).sum())

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!("1 with lookup function", part_one, input);
    advent_of_code::solve!("2 with lookup function", part_two, input);

    advent_of_code::solve!("1 with hashmap", part_one_clean_but_slow, input);
    advent_of_code::solve!("2 with hashmap", part_two_clean_but_slow, input);
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
