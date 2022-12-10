
pub fn part_one(input: &str) -> Option<i16> {
    let mut cycle: i16 = 1;
    let mut val: i16 = 1;

    //let mut com: &str;
    let mut num: &str;

    let mut num_vec: Vec<i16> = vec![];
    let targets: Vec<i16> = vec![20, 60, 100, 140, 180, 220];

    for line in input.lines(){
        // check if in target always
        if targets.contains(&cycle){
            num_vec.push(val);
        }
        cycle += 1;
        if line!="noop"{
            (_, num) = line.split_once(' ').unwrap();
            // check second time if in target if com takes two cycles
            if targets.contains(&cycle){
                num_vec.push(val);
            }
            cycle += 1;
            // add the value
            val += num.parse::<i16>().unwrap();
        }
    }
    // dot product of targets and num_vec
    Some(targets.iter()
         .zip(num_vec.iter())
         .map(|(x, y)| x * y)
         .sum::<i16>()
    )

}

pub fn part_two_op_print(input: &str, print: bool) -> Option<&str> {
    let mut cycle: i16 = 0;
    let mut eff_cycle: i16 = 0;

    let mut num: &str;
    let mut result: Vec<char> = vec!['.';240];
    //let mut result: [&str; 240] = ['#'; 240];
    let mut sprite_center: i16 = 1;

    for line in input.lines(){
        if (eff_cycle-sprite_center).abs()<2{
            // draw the pixel as lit
            result[cycle as usize] = '#';
        }
        cycle += 1;
        eff_cycle += 1;
        if eff_cycle >= 40{
            eff_cycle -= 40
        }
        if line!="noop"{
            (_, num) = line.split_once(' ').unwrap();
            // check second time if in target if com takes two cycles
            if (eff_cycle-sprite_center).abs()<2{
                // draw the pixel as lit
                result[cycle as usize] = '#';
            }
            cycle += 1;
            eff_cycle += 1;
            if eff_cycle >= 40{
                eff_cycle -= 40
            }
            // add the value
            sprite_center += num.parse::<i16>().unwrap();
        }
    }
    if print{
        for i in 0..6{
            println!("{:?}", &result[i*40..(i+1)*40]);
        }
    }
    Some("yes")
}
pub fn part_two(input: &str) -> Option<&str> {
    part_two_op_print(input, true)
}

pub fn part_two_skip_print(input: &str) -> Option<&str> {
    part_two_op_print(input, false)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10, None);
    advent_of_code::solve!(1, part_one, input);
    //advent_of_code::solve!("2 with print", part_two, input);
    advent_of_code::solve!("2 without print", part_two_skip_print, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10, None);
        assert_eq!(part_one(&input), Some(13140));
    }

}
