use nalgebra::DMatrix;


pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

pub fn check(mat: &DMatrix<u32>, row: usize, col: usize, k: usize) -> bool {
    if k==0{
        return mat[(row, col)] > mat.column(col).slice_range(..row, 0..1).max();
    }
    if k==1{
        return mat[(row, col)] > mat.row(row).slice_range(0..1, ..col).max();
    }
    if k==2{
        return mat[(row, col)] > mat.column(col).slice_range(row+1.., 0..1).max();
    }
    if k==3{
        return mat[(row, col)] > mat.row(row).slice_range(0..1, col+1..).max();
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let n = input.lines().count();
    let binding = input.replace('\n', "");
    let numbers: Vec<u32> = binding
                       .chars()
                       .map(|c| c.to_digit(10).unwrap())
                       .collect();

    let mat = DMatrix::from_row_slice(n, n, &numbers);

    let mut visible: usize = 2*n+2*(n-2);
    let mut which_way: Vec<usize>;
    for row in 1..n-1{
        for col in 1..n-1{
            // which way to check first?
            which_way = argsort(&[row,col,n-row,n-col]);
            for k in which_way{
                if check(&mat, row, col, k){
                    visible+=1;
                    break;
                }
            }
        }
    }
    Some(visible)
}

pub fn score(mat: &DMatrix<u32>, row: usize, col: usize) -> u32 {
    let mut score: u32 = 1;
    let mut sub_score: u32 = 0;
    let val = &mat[(row, col)];
    for e in mat.column(col).slice_range(..row, 0..1).iter().rev(){
        sub_score += 1;
        if e>=val{
            break;
        }
    }
    score *= sub_score;
    sub_score=0;
    for e in mat.row(row).slice_range(0..1, ..col).iter().rev(){
        sub_score += 1;
        if e>=val{
            break;
        }
    }
    score *= sub_score;
    sub_score=0;
    for e in mat.column(col).slice_range(row+1.., 0..1).iter(){
        sub_score += 1;
        if e>=val{
            break;
        }
    }
    score *= sub_score;
    sub_score=0;
    for e in mat.row(row).slice_range(0..1, col+1..).iter(){
        sub_score += 1;
        if e>=val{
            break;
        }
    }
    score *= sub_score;
    score
}

pub fn part_two(input: &str) -> Option<u32> {
    let n = input.lines().count();
    let binding = input.replace('\n', "");
    let numbers: Vec<u32> = binding
                       .chars()
                       .map(|c| c.to_digit(10).unwrap())
                       .collect();

    let mat = DMatrix::from_row_slice(n, n, &numbers);
    let mut max_score: u32 = 0;
    let mut this_score: u32;
    for row in 1..n-1{
        for col in 1..n-1{
            // which way to check first?
            this_score = score(&mat, row, col);
            if this_score>max_score{
                max_score=this_score;
            }
        }
    }
    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    //part_two(input);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
