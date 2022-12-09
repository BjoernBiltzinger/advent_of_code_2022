pub fn start_stops(input1: &str, input2: &str) -> (u16, u16, u16, u16) {
    let (start1, stop1) = input1.split_once('-').unwrap();
    let (start2, stop2) = input2.split_once('-').unwrap();

    ((start1.parse().unwrap()), (stop1.parse().unwrap()),
     (start2.parse().unwrap()), (stop2.parse().unwrap()))
}

pub fn check_contains(start1: u16, stop1: u16, start2: u16, stop2: u16) -> u16 {
    if start1>=start2 && stop1<=stop2{
        return 1;
    }
    if start2>=start1 && stop2<=stop1{
        return 1;
    }
    0
}

pub fn check_overlap(start1: u16, stop1: u16, start2: u16, stop2: u16) -> u16 {
    if start1>=start2 && start1<=stop2{
        return 1;
    }
    if start2>=start1 && start2<=stop1{
        return 1;
    }
    0
}
pub fn part_one(input: &str) -> Option<u16> {
    Some(input.lines().map(|l| {
        let (split1, split2) = l.split_once(',').unwrap();
        let (start1, stop1, start2, stop2) = start_stops(split1, split2);
        check_contains(start1, stop1, start2, stop2)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u16> {
    Some(
        input.lines().map(|l| {
            let mut split = l.split(',');
            let (start1, stop1, start2, stop2) = start_stops(split.next().unwrap(),
                                                             split.next().unwrap());
            check_overlap(start1, stop1, start2, stop2)
        }).sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4, None);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4, None);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4, None);
        assert_eq!(part_two(&input), Some(4));
    }
}
