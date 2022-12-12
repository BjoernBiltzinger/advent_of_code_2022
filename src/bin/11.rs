use itertools::Itertools;

//taken from fspoettel

#[derive(Clone)]
#[derive(Debug)]
enum Operand {
    OldValue,
    Number(u64),
}

impl Operand {
    fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "old" => Some(Operand::OldValue),
            x => Some(Operand::Number(x.parse().ok()?)),
        }
    }

    fn apply(&self, x: u64) -> u64 {
        match self {
            Operand::OldValue => x,
            Operand::Number(y) => *y,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(a, b) => a.apply(x) + b.apply(x),
            Self::Multiply(a, b) => a.apply(x) * b.apply(x),
        }
    }
}

//#[derive(Clone)]
#[derive(Debug)]
pub struct MONKEY{
    items: Vec<u64>,
    oper: Operation,
    test_num: u64,
    true_next_monkey: usize,
    false_next_monkey: usize,
}

pub fn get_start(input: &str) -> Vec<MONKEY>{

    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|mut chunk|{
            chunk.next();
            let (_, list) = chunk.next().unwrap().split_once(": ").unwrap();
            let mut item_list: Vec<u64> = vec![];
            for l in list.split(", "){
                item_list.push(l.parse::<u64>().unwrap());
            }
            //let (_, operation) = chunk.next().unwrap().split_once("old ").unwrap();
            let mut operation_line = chunk.next().unwrap().split_once("= ").unwrap().1.split_ascii_whitespace();
            let operand_a = operation_line.next().map(Operand::try_from_str).unwrap().unwrap();
            let operator = operation_line.next().unwrap();
            let operand_b = operation_line.next().map(Operand::try_from_str).unwrap().unwrap();
            let operation = match operator {
                "+" => Operation::Add(operand_a, operand_b),
                "*" => Operation::Multiply(operand_a, operand_b),
                _ => unreachable!(),
            };
            let test_num = chunk.next().unwrap()
                                       .split_ascii_whitespace()
                                       .last()
                                       .unwrap()
                                       .parse()
                                       .unwrap();
            let true_next_monkey = chunk.next().unwrap()
                                       .split_ascii_whitespace()
                                       .last()
                                       .unwrap()
                                       .parse()
                                       .unwrap();
            let false_next_monkey = chunk.next().unwrap()
                                       .split_ascii_whitespace()
                                       .last()
                                       .unwrap()
                                       .parse()
                                       .unwrap();
            MONKEY{items: item_list,
                   oper: operation,
                   test_num: test_num,
                   true_next_monkey: true_next_monkey,
                   false_next_monkey: false_next_monkey}
        }).collect()

}

pub fn simulate(input: &str, rounds: usize, func: impl Fn(u64) -> u64, part: u8) -> usize {

    let mut monkeys: Vec<MONKEY> = get_start(input);
    let mut norm: u64 = 0;
    if part==2{
        norm = monkeys.iter().map(|x| x.test_num).product();
    }
    //println!("{:?}", monkeys);
    let mut nums = vec![0; monkeys.len()];
    for _ in 0..rounds{
        for m in 0..monkeys.len(){
            let mut monkey = &mut monkeys[m];

            let items: Vec<u64>;
            if part==2{
                items = monkey
                .items
                .drain(..)
                .map(|item| func(monkey.oper.apply(item)) % norm)
                .collect();
            } else{
                items = monkey
                .items
                .drain(..)
                .map(|item| func(monkey.oper.apply(item)))
                .collect();
            }
            nums[m] += items.len();

            let divide_by = monkey.test_num;
            let next_truthy = monkey.true_next_monkey;
            let next_falsy = monkey.false_next_monkey;
            for item in items{
                let target = if item % divide_by == 0 {
                    next_truthy
                } else {
                    next_falsy
                };
                monkeys[target].items.push(item);
            }
            //println!("{:?}", nums.iter().sorted().rev().take(2));
        }
    }
    nums.sort_unstable();

    nums.iter().sorted().rev().take(2).product()
    //println!("{:?}", monkeys);
    //None
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(simulate(input, 20, |x| x / 3, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    // all division are prime numbers
    // modulu with the product of these will not change the real
    // modulu result
    Some(simulate(input, 10000, |x| x, 2))
}

fn main() {

    //let m = MONKEY{items: vec![10,12,15], op_type: '+',
    //               op_num: 10, test_num: 10,
    //               true_next_monkey: 3,
    //               false_next_monkey: 1};
    //println!("{:?}", &m.operation(10));
    //println!("{:?}", &m.next_monky_index(20));
    let input = &advent_of_code::read_file("inputs", 11, None);
    //part_one(input);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11, None);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11, None);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
