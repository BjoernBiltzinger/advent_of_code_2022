use itertools::Itertools;

pub fn value(input: &char) -> u32 {
    let mut val = input.to_digit(36).unwrap()-9;
    if input.is_ascii_uppercase(){
        val += 26;
    }
    val
}

pub fn find_char1(string1: &str, string2: &str) -> char {
    let mut idx: Option<usize>;
    for c in string1.chars(){
        idx = string2.find(c);
        if idx.is_some(){
            return c;
        }
    }
    println!("{} AND {}", string1, string2);
    panic!("This should never happen!");
}

pub fn find_char2(string1: &str, string2: &str, string3: &str) -> char {
    let mut idx: Option<usize>;
    let mut idx2: Option<usize>;
    for c in string1.chars(){
        idx = string2.find(c);
        if idx.is_some(){
            idx2 = string3.find(c);
            if idx2.is_some(){
                return c;
            }
        }
    }
    println!("{} AND {} AND {}", string1, string2, string3);
    panic!("This should never happen!");
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|l| {
            let length = l.len()/2;
            if length > 0 {
                let (string1, string2) = l.split_at(length);
                let c = find_char1(string1, string2);
                value(&c)
            } else {
                0
            }
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines().chunks(3).into_iter().map(
            |mut ls| {
                let c = find_char2(ls.next().unwrap(),
                                   ls.next().unwrap(),
                                   ls.next().unwrap());
                value(&c)
            }
        ).sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
