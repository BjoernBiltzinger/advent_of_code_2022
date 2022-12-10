use itertools::Itertools;

pub fn move_head(mut current_pos: (i16, i16), dir: &str) -> (i16, i16){
    match dir {
        "U" => current_pos.1 += 1,
        "D" => current_pos.1 -= 1,
        "R" => current_pos.0 += 1,
        "L" => current_pos.0 -= 1,
        _ => unreachable!()
    };
    current_pos
}

pub fn move_tail(mut current_pos: (i16, i16), head_pos: &(i16, i16), dir: &str) -> (i16, i16){

    let x_diff = head_pos.0-current_pos.0;
    let y_diff = head_pos.1-current_pos.1;

    // all the different combination possibilities with nested match statements
    // no idea if this is efficient
    match x_diff{
        2 => match y_diff{
            1 | 2 => {current_pos.0+=1; current_pos.1+=1},
            0 => {current_pos.0+=1;},
            -1 | -2 => {current_pos.0+=1; current_pos.1-=1},
            _ => unreachable!(),
        },
        1 => match y_diff{
            2 => {current_pos.0+=1; current_pos.1+=1},
            -2 => {current_pos.0+=1; current_pos.1-=1},
            1 | -1 | 0 => (), //nothing
            _ => unreachable!(),
        },
        0 => match y_diff{
            0 | 1 | -1 => (),//nothing
            2 => {current_pos.1+=1;},
            -2 => {current_pos.1-=1;},
            _ => unreachable!(),
        },
        -1 =>match y_diff{
            2 => {current_pos.0-=1; current_pos.1+=1},
            -2 => {current_pos.0-=1; current_pos.1-=1},
            1 | -1 | 0 => (), //nothing
            _ => unreachable!(),
        },
        -2 => match y_diff{
            1 | 2 => {current_pos.0-=1; current_pos.1+=1},
            0 => {current_pos.0-=1;},
            -1 | -2 => {current_pos.0-=1; current_pos.1-=1},
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    current_pos
}

pub fn simulate(input: &str, num_knots: usize) -> Option<usize>{

    let mut all_tail_pos: Vec<(i16,i16)> = vec![(0,0)];

    let mut knot_positions: Vec<(i16, i16)> = vec![(0,0); num_knots];
    let mut dir: &str;
    let mut num: &str;
    for line in input.lines(){

        (dir, num) = line.split_once(' ').unwrap();
        for _ in 0..num.parse::<usize>().unwrap(){
            // update head position
            knot_positions[0] = move_head(knot_positions[0], dir);
            // iteratively update the following positions
            for i in 1..num_knots{
                knot_positions[i] = move_tail(knot_positions[i],
                                              &knot_positions[i-1],
                                              dir);
            }
            // save the tail positions
            all_tail_pos.push(knot_positions[num_knots-1]);
        }
    }
    // get the number of unique tail positions
    Some(all_tail_pos.into_iter().unique().count())
}

pub fn part_one(input: &str) -> Option<usize> {
    simulate(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    simulate(input, 10)
}

fn main() {

    let input = &advent_of_code::read_file("inputs", 9, None);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9, None);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9, Some(2));
        assert_eq!(part_two(&input), Some(36));
    }
}
