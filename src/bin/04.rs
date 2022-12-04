pub fn start_stops(input1: &str, input2: &str) -> (u32, u32, u32, u32) {
    let mut split1 = input1.split("-");

    let (start1, stop1): (u32, u32) = (split1.next().unwrap().parse().unwrap(),
                                       split1.next().unwrap().parse().unwrap());
    let mut split2 = input2.split("-");
    let (start2, stop2): (u32, u32) = (split2.next().unwrap().parse().unwrap(),
                                       split2.next().unwrap().parse().unwrap());
    (start1, stop1, start2, stop2)
}

pub fn check_contains(start1: u32, stop1: u32, start2: u32, stop2: u32) -> u32 {
    if start1>=start2 && stop1<=stop2{
        return 1;
    }
    if start2>=start1 && stop2<=stop1{
        return 1;
    }
    return 0
}

pub fn check_overlap(start1: u32, stop1: u32, start2: u32, stop2: u32) -> u32 {
    if start1>=start2 && start1<=stop2{
        return 1;
    }
    if start2>=start1 && start2<=stop1{
        return 1;
    }
    return 0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|l| {
            let mut split = l.split(",");
            let (start1, stop1, start2, stop2) = start_stops(split.next().unwrap(),
                                                             split.next().unwrap());
            check_contains(start1, stop1, start2, stop2)
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|l| {
            let mut split = l.split(",");
            let (start1, stop1, start2, stop2) = start_stops(split.next().unwrap(),
                                                             split.next().unwrap());
            check_overlap(start1, stop1, start2, stop2)
        }).sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    //let (start1, stop1, start2, stop2) = start_stops("1-9", "3-8");
    //println!("{:?} {:?} {:?} {:?}", start1, stop1, start2, stop2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
