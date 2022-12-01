/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn sum_list(input: &str) -> u32 {

    let nums = input.split("\n");
    let mut sum: u32 = 0;
    for n in nums{
        if !n.is_empty() {
            sum += n.parse::<u32>().unwrap();
        }
    }
    sum
}
