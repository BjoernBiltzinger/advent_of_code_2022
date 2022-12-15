use regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Beacon{
    x: isize,
    y: isize
}

#[derive(Debug)]
pub struct Sender{
    x: isize,
    y: isize,
    closest_beacon: Beacon
}

impl Sender{

    pub fn new(x: isize, y: isize, beacon_x: isize, beacon_y: isize) -> Self{
        Self {x, y, closest_beacon: Beacon{x: beacon_x, y: beacon_y}}
    }

    pub fn distance_to_beacon(&self) -> isize {
        (self.x-self.closest_beacon.x).abs()+(self.y-self.closest_beacon.y).abs()
    }
}

#[derive(Debug, Clone)]
struct Range{
    start: isize,
    end: isize,
}

#[derive(Debug, Clone)]
pub struct AllRanges{
    ranges: Vec<Range>,
}

impl AllRanges{
    pub fn new() -> Self {
        Self {ranges: vec![]}
    }

    pub fn add_range(self, start: isize, end: isize) -> AllRanges{
        // new object
        let mut new = AllRanges{ranges: vec![]};

        let mut new_start = start;
        let mut new_end = end;

        let mut add: bool = true;
        // check if this range overlaps with a already saved range
        for r in self.ranges{
            if start>=r.start && end<=r.end {
                // covered completly by the other range
                // dont do anything
                add = false;
                new.ranges.push(r);
            }
            else if start<=r.end+1 && end>=r.end+1{
                // start during a previous range but is longer
                if r.start<new_start{
                    new_start = r.start;
                }
            }
            else if start<=r.start-1 && end>=r.start-1{
                // end during a previous range but is longer
                if r.end>new_end{
                    new_end = r.end;
                }
            }
            else if start<=r.start && end>=r.end{
                // old range completly covered
                // => drop old range
                continue;
            }
            else{
                // untouched
                new.ranges.push(r);
            }
        }
        if add{
            new.ranges.push(Range{start: new_start, end: new_end});
        }
        new

    }
    pub fn total_length(&self) -> isize {
        self.ranges.iter().map(|r| r.end-r.start+1).sum()
    }
}

lazy_static! {
    static ref re: Regex = Regex::new(r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)").unwrap();
}

pub fn read_line(line: &str) -> Sender {
    let cap = re.captures(line).unwrap();
    Sender::new(cap[1].parse::<isize>().unwrap(),
                cap[2].parse::<isize>().unwrap(),
                cap[3].parse::<isize>().unwrap(),
                cap[4].parse::<isize>().unwrap())
}

pub fn part_one(input: &str) -> Option<isize> {
    let target_line = 10;//2000000;
    let senders = input.lines().map(|l| read_line(l));
    //let mut hit_line: Vec<isize> = vec![];
    let mut ranges: AllRanges = AllRanges::new();
    for s in senders{

        let min_distance = s.distance_to_beacon();
        let left_for_x = min_distance - (target_line-s.y).abs();
        if left_for_x>0 {
            // will have some hits
            ranges = ranges.add_range(s.x-left_for_x, s.x+left_for_x);

        }
    }

    // one beacon in this row
    Some(ranges.total_length()-1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15, None);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15, None);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15, None);
        assert_eq!(part_two(&input), None);
    }
}
