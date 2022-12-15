//use hashbrown::HashMap;

#[derive(Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// A point in a 2D grid. Uses `isize` to support use in sparse grids where points may be negative.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    //pub fn manhattan_distance(&self, other: &Point) -> isize {
    //    (self.x - other.x).abs() + (self.y - other.y).abs()
    //}

    //pub fn chebyshev_distance(&self, other: &Point) -> isize {
    //    cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    //}

    /// Get point x steps away in a given direction.
    #[inline(always)]
    pub fn get_neighbour(&self, direction: &Direction, steps: isize) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - steps,
            },
            Direction::NorthEast => Self {
                x: self.x + steps,
                y: self.y - steps,
            },
            Direction::East => Self {
                x: self.x + steps,
                y: self.y,
            },
            Direction::SouthEast => Self {
                x: self.x + steps,
                y: self.y + steps,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + steps,
            },
            Direction::SouthWest => Self {
                x: self.x - steps,
                y: self.y + steps,
            },
            Direction::West => Self {
                x: self.x - steps,
                y: self.y,
            },
            Direction::NorthWest => Self {
                x: self.x - steps,
                y: self.y - steps,
            },
        }
    }
}

//#[derive(Clone)]
//pub struct SparseGrid<T> {
//    pub points: HashMap<Point, T>,
//}
#[derive(Clone, Debug)]
pub struct SparseGrid {
    pub points: Vec<Point>,
}

//impl<T> SparseGrid<T> {
impl SparseGrid{
    //pub fn get(&self, point: &Point) -> Option<&T> {
    //    self.points.get(point)
    //}

    //pub fn insert(&mut self, point: Point, data: T) {
    //    self.points.insert(point, data);
    //}
    pub fn insert(&mut self, point: Point){
        if !self.points.contains(&point){
            self.points.push(point)
        }
    }

    pub fn ymin(&self) -> isize {
        self.points.iter().map(|p| p.y).min().unwrap()
    }

    pub fn ymax(&self) -> isize {
        self.points.iter().map(|p| p.y).max().unwrap()
    }

    pub fn xmin(&self) -> isize {
        self.points.iter().map(|p| p.x).min().unwrap()
    }

    pub fn xmax(&self) -> isize {
        self.points.iter().map(|p| p.x).max().unwrap()
    }
}

//impl<T> Default for SparseGrid<T> {
impl Default for SparseGrid {
    fn default() -> Self {
        Self {
            points: vec![]//HashMap::new(),
        }
    }
}


pub fn add_points(mut grid: SparseGrid, start_tuple: (isize, isize), end_tuple: (isize, isize))  -> SparseGrid{
    let xdiff = end_tuple.0-start_tuple.0;
    let ydiff = end_tuple.1-start_tuple.1;
    let mut current_point = Point{x: start_tuple.0, y: start_tuple.1};
    grid.insert(current_point.clone());
    if xdiff == 0 {
        if ydiff>=0{
            for _ in 1..=ydiff{
                current_point = current_point.get_neighbour(&Direction::South, 1);
                grid.insert(current_point.clone());
            }
        } else {
            for _ in ydiff..0{
                current_point = current_point.get_neighbour(&Direction::North, 1);
                grid.insert(current_point.clone());
            }
        }
    } else {
        if xdiff>=0{
            for _ in 1..=xdiff{
                current_point = current_point.get_neighbour(&Direction::East, 1);
                grid.insert(current_point.clone());
            }
        } else {
            for _ in xdiff..0{
                current_point = current_point.get_neighbour(&Direction::West, 1);
                grid.insert(current_point.clone());
            }
        }
    }
    grid
}

pub fn parse_line(line: &str) -> Vec<(isize, isize)>{
    let mut v = vec![];
    let split = line.split("->");
    for s in split{
        let (n1, n2) = s.split_once(',').unwrap();
        v.push((n1.trim().parse::<isize>().unwrap(),
                n2.trim().parse::<isize>().unwrap()));
    }
    v
}

pub fn drop_sand(mut grid: SparseGrid, point: &Point, y_max: isize, x_min: isize, x_max: isize)  -> (SparseGrid, bool){

    if point.y > y_max || point.x<x_min || point.x>x_max {
        // nope
        return (grid, false);
    }

    // check below
    let below = point.get_neighbour(&Direction::South, 1);
    if !grid.points.contains(&below){
        // go there
        return drop_sand(grid, &below, y_max, x_min, x_max);
    }
    let below_left = point.get_neighbour(&Direction::SouthWest, 1);
    if !grid.points.contains(&below_left){
        // go there
        return drop_sand(grid, &below_left, y_max, x_min, x_max);
    }
    let below_right = point.get_neighbour(&Direction::SouthEast, 1);
    if !grid.points.contains(&below_right){
        // go there
        return drop_sand(grid, &below_right, y_max, x_min, x_max);
    }
    // stop here
    grid.insert(point.clone());
    (grid, true)
}

pub fn drop_sand_two(mut grid: SparseGrid, point: &Point, floor: isize)  -> (SparseGrid, bool){

    if point.y == floor{
        // stop
        grid.insert(point.clone());
        return (grid, true)
    }
    // check below
    let below = point.get_neighbour(&Direction::South, 1);
    if !grid.points.contains(&below){
        // go there
        return drop_sand_two(grid, &below, floor);
    }
    let below_left = point.get_neighbour(&Direction::SouthWest, 1);
    if !grid.points.contains(&below_left){
        // go there
        return drop_sand_two(grid, &below_left, floor);
    }
    let below_right = point.get_neighbour(&Direction::SouthEast, 1);
    if !grid.points.contains(&below_right){
        // go there
        return drop_sand_two(grid, &below_right, floor);
    }
    // stop here
    grid.insert(point.clone());
    (grid, true)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = SparseGrid::default();
    for l in input.lines(){

        let knots = parse_line(l);
        for i in 0..knots.len()-1{
            grid = add_points(grid.clone(), knots[i], knots[i+1]);
            //println!("{:?}", grid);
        }
        //unreachable!();
    }
    let sand_start_pos = Point{x: 500, y: 0};
    let mut stopped: bool;
    let mut num: u32 = 0;
    loop {
        // sand
        let ymax = grid.ymax();
        let xmax = grid.xmax();
        let xmin = grid.xmin();
        (grid, stopped) = drop_sand(grid, &sand_start_pos, ymax, xmin, xmax);
        //println!("{:?}", grid.points.len());
        if !stopped{
            return Some(num);
        }
        num += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = SparseGrid::default();
    for l in input.lines(){

        let knots = parse_line(l);
        for i in 0..knots.len()-1{
            grid = add_points(grid.clone(), knots[i], knots[i+1]);
            //println!("{:?}", grid);
        }
        //unreachable!();
    }
    let sand_start_pos = Point{x: 500, y: 0};
    let mut stopped: bool;
    let mut num: u32 = 0;
    let floor = grid.ymax()+1;
    loop {
        // sand
        (grid, stopped) = drop_sand_two(grid, &sand_start_pos, floor);
        //println!("{:?}", grid.points.len());
        num += 1;
        if grid.points.contains(&sand_start_pos){
            return Some(num);
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14, None);
    //let input = &advent_of_code::read_file("examples", 14, None);
    //part_one(input);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14, None);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14, None);
        assert_eq!(part_two(&input), None);
    }
}
