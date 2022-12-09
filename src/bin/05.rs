use regex::Regex;

pub fn get_start_vec(input: &str) -> Vec<(String)> {
    let lines= input.lines().rev().skip(1).map(|l| l.chars());
    let num_columns = (&lines.clone().next().unwrap().count()+1)/4;
    let mut crates = Vec::new();
    for i in 0..num_columns{
        crates.push(String::from(""))
    }
    let mut c: Option<char>;
    for col in (0..num_columns) {

        for mut line in lines.clone() {
            c = line.nth(col*4+1);
            if c.is_some(){
                if c.unwrap() != ' '{
                    crates.get_mut(col).unwrap().push(c.unwrap());
                }
            } else {
                break
            }
        }
    }
    crates
}

pub fn parse_command_slow(command: &str) -> (u8, usize, usize) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let cap = re.captures(command).unwrap();
    (cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
     cap.get(2).unwrap().as_str().parse::<usize>().unwrap()-1,
     cap.get(3).unwrap().as_str().parse::<usize>().unwrap()-1,
    )
}

pub fn parse_command(command: &str) -> (u8, usize, usize) {
    let s: Vec<&str> = command.split_ascii_whitespace().collect();
    (s[1].parse::<u8>().unwrap(),
     s[3].parse::<usize>().unwrap()-1,
     s[5].parse::<usize>().unwrap()-1)
}

pub fn update_crates(mut crates: &mut Vec<(String)>, amount: u8, from: usize, to: usize, part: u8){
    let from_string = crates.get_mut(from).unwrap();
    let mut move_string: String = String::from("");

    for _ in 0..amount {
        move_string.push(from_string.pop().unwrap());
    }
    let to_string = crates.get_mut(to).unwrap();
    //let chars = move_string.chars();
    if part == 1 {
        for c in move_string.chars(){
            to_string.push(c);
        }
    } else {
        for c in move_string.chars().rev(){
            to_string.push(c);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (initial, commands) = input.split_once("\n\n").unwrap();
    let mut crates = get_start_vec(initial);
    let (mut from, mut to, mut amount): (usize, usize, u8);
    for com in commands.lines() {
        (amount, from, to) = parse_command(com);
        update_crates(&mut crates, amount, from, to, 1);
    }

    let mut result = String::from("");
    for mut c in crates{
        result.push(c.pop().unwrap());
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (initial, commands) = input.split_once("\n\n").unwrap();
    let mut crates = get_start_vec(initial);
    let (mut from, mut to, mut amount): (usize, usize, u8);
    for com in commands.lines() {
        (amount, from, to) = parse_command(com);
        update_crates(&mut crates, amount, from, to, 2);
    }

    let mut result = String::from("");
    for mut c in crates{
        result.push(c.pop().unwrap());
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5, None);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5, None);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5, None);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
