use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub enum DIRECTION{
    UP,
    DOWN,
    RIGHT,
    LEFT
}

pub struct POINT{
    x: isize,
    y: isize,
}

impl POINT{
    pub fn get_neighbour(&self, direction: DIRECTION, num: isize) -> POINT{
        match direction{
            DIRECTION::UP => Self{
                x: self.x,
                y: self.y+num
            },
            DIRECTION::DOWN => Self{
                x: self.x,
                y: self.y-num
            },
            DIRECTION::LEFT => Self{
                x: self.x-num,
                y: self.y
            },
            DIRECTION::RIGHT => Self{
                x: self.x+num,
                y: self.y
            },
        }
    }
}

pub struct GRID<T>{
    width: usize,
    heigth: usize,
    data: Vec<Vec<T>>, //vec of vec => matrix
}

impl<T> GRID<T>{
    pub fn get_data(&self, point: &POINT) -> &T {
        &self.data[point.y as usize][point.x as usize]
    }

    /// Check if a point is at the boundary of the grid.
    pub fn at_boundary(&self, point: &POINT) -> bool {
        point.x == 0
            || point.y == 0
            || point.x as usize == self.width - 1
            || point.y as usize == self.heigth - 1
    }

    /// Check if a point is inside grid.
    pub fn is_inside(&self, point: &POINT) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.width
            && (point.y as usize) < self.heigth
    }

    pub fn from_str(input: &str, parse: &mut dyn FnMut(char, usize, usize) -> T) -> Self {
        // map lines into a nested list, applying parser to each item.
        let data: Vec<Vec<T>> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| l.chars().enumerate().map(|(x, c)| parse(c, x, y)).collect())
            .collect();

        GRID {
            width: data[0].len(),
            heigth: data.len(),
            data,
        }
    }

    pub fn id_point(&self, p: &POINT) -> usize {
        p.x as usize + self.width * p.y as usize
    }

    pub fn point_id(&self, id: usize) -> POINT {
        POINT {
            x: (id % self.width) as isize,
            y: (id / self.width) as isize,
        }
    }
    pub fn points(&self) -> Vec<POINT> {
        let mut points = vec![];

        for y in 0..self.heigth {
            for x in 0..self.width {
                points.push(POINT {
                    x: x as isize,
                    y: y as isize,
                })
            }
        }

        points
    }
}

// nope: recursion with structs is really hard in rust
/*
pub fn call<'a>(point: &POINT, value_map: &GRID<u16>,
                mut cost_map: &'a mut GRID<u16>, mut current_cost: u16) -> &'a mut GRID<u16>{
    // get this value
    let val = value_map.get_data(point);
    // add to current_cost
    current_cost = current_cost + val;
    // if current_cost now > cost_map of this point => already know better way here
    // => drop this path
    if current_cost>*cost_map.get_data(point){
        return cost_map;
    } else {
        cost_map.get_data(point) = current_cost;
    }
    let mut new_point: POINT;
    for dir in [DIRECTION::UP, DIRECTION::DOWN, DIRECTION::RIGHT, DIRECTION::LEFT]{
        new_point = point.get_neighbour(dir, 1);
        // check if new point is in bound
        if value_map.is_inside(&new_point){
            // check if value difference not too large
            if value_map.get_data(&new_point)-val < 2{
                cost_map = call(&new_point, value_map, cost_map, current_cost);
            }
        }
    }
    return cost_map;
}
 */

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BEST{
    cost: usize,
    position: usize
}
impl Ord for BEST {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
}
impl PartialOrd for BEST {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

pub fn get_shortest(grid: &GRID<usize>, start_points: Vec<POINT>, end_point: POINT) -> Option<usize> {
    // read in vec<vec<char>> turn to vec<vec<u16>> && find start and end position

    let mut res = BinaryHeap::new();
    let mut dist: Vec<_> = (0..(grid.width * grid.heigth))
            .map(|_| usize::MAX)
            .collect();

    //let start_id = grid.id_point(&start_point.unwrap());
    let end_id = grid.id_point(&end_point);
    //dist[start_id] = 0;
    for start_point in start_points {
        let start_id = grid.id_point(&start_point);
        dist[start_id] = 0;
        res.push(BEST {
            cost: 0,
            position: start_id,
        });
    }

    //while points.
    while let BEST { cost, position } = res.pop().unwrap() {
        if position == end_id {
            return Some(cost);
        }

        if cost>dist[position]{
            continue;
        }

        let point = grid.point_id(position);
        for dir in [DIRECTION::UP, DIRECTION::DOWN, DIRECTION::RIGHT, DIRECTION::LEFT]{
            let mut new_point = point.get_neighbour(dir, 1);
            // check if new point is in bound
            if grid.is_inside(&new_point){
                if (*grid.get_data(&new_point) as isize - *grid.get_data(&point) as isize) < 2{
                    let next = BEST {
                        cost: cost + 1, //grid.get_data(&new_point),
                        position: grid.id_point(&new_point)
                    };

                    if next.cost < dist[next.position]{
                        res.push(next);
                        dist[next.position] = next.cost;
                    }
                }
            }
        }
    }
    None
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut start: Option<POINT> = None;
    let mut end: Option<POINT> = None;

    let grid: GRID<usize> = GRID::from_str(input, &mut |c, x, y| match c {
        'S' => {
            start = Some(POINT {
                x: x as isize,
                y: y as isize,
            });
            0
        }
        'E' => {
            end = Some(POINT {
                x: x as isize,
                y: y as isize,
            });
            26
        }
        c => c.to_digit(36).unwrap() as usize -10,
    });
    get_shortest(&grid, vec![start.unwrap()], end.unwrap())
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut start: Option<POINT> = None;
    let mut end: Option<POINT> = None;

    let grid: GRID<usize> = GRID::from_str(input, &mut |c, x, y| match c {
        'S' => {
            start = Some(POINT {
                x: x as isize,
                y: y as isize,
            });
            0
        }
        'E' => {
            end = Some(POINT {
                x: x as isize,
                y: y as isize,
            });
            26
        }
        c => c.to_digit(36).unwrap() as usize -10,
    });
    let start_points: Vec<POINT> = grid
        .points()
        .into_iter()
        .filter(|point| *grid.get_data(point) == 0)
        .collect();
    get_shortest(&grid, start_points, end.unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12, None);

    //part_one(input);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12, None);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12, None);
        assert_eq!(part_two(&input), None);
    }
}
