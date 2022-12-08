use petgraph::graph::{NodeIndex, Graph};
use petgraph::Direction;
use petgraph::dot::{Dot, Config};
//use crate::Direction::Ingoing;
//use crate::Direction::Outgoing;

#[derive(Debug)]
struct FILE{
    //name: &'static str,
    size: u32,
}

fn get_sum_dirs<'a>(g: &Graph::<(&'static str, Vec<u32>), ()>, current_node: NodeIndex) -> (u32, u32){
    // recursive call to child nodes
    let (mut sum, mut valid_sum) = (0, 0);
    for n in g.neighbors_directed(current_node,
                                  Direction::Outgoing){
        (sum, valid_sum) = get_sum_dirs(&g, n);
    }

    // add files of this node to the sum
    sum += &g[current_node].1.iter().sum::<u32>();
    if sum<=100000{
        valid_sum+=sum;
    }
    (sum, valid_sum)
}

fn get_graph<'a>(input: &'a str) -> (Graph::<(&'a str, Vec<u32>), ()>, NodeIndex){
    let mut g = Graph::<(&'static str, Vec<u32>), ()>::new();
    let mut com: &str;
    let mut w: &str;
    let mut base_node = g.add_node(("/", Vec::new()));
    let mut current_node = base_node;
    let mut new_node: NodeIndex;
    let mut found_existing: bool;
    let mut size: &str;
    for line in input.lines().skip(1){
        found_existing = false;
        if line.starts_with('$'){
            // command
            (_, com) = line.split_once(' ').unwrap();
            if com != "ls"{
                //println!("{}", line);
                // changing directory
                (_, w) = com.split_once(' ').unwrap();
                if w == ".."{
                    //println!("do ..");
                    //go to parent
                    current_node = g.neighbors_directed(current_node,
                                                        Direction::Incoming).last().unwrap();
                } else if w=="/"{
                    //println!("do /");
                    // go to root
                    current_node = base_node;

                } else {
                    //println!("do {}", w);
                    // go to child
                    for n in g.neighbors_directed(current_node,
                                                  Direction::Outgoing)
                    {
                        if g[n].0 == w{
                            current_node = n;
                            found_existing = true;
                            break;
                        }
                    }
                    if !found_existing {
                        // new node
                        new_node = g.add_node((w, Vec::new()));

                        // add connection
                        g.add_edge(current_node, new_node, ());

                        // update current node
                        current_node = new_node;
                    }
                }
            }
        } else if line.starts_with("dir"){
            continue;
        } else {
            // this is file, add it to the current node
            (size, _) = line.split_once(' ').unwrap();
            g[current_node].1.push(size.clone().parse::<u32>().unwrap());
            //g[current_node].1.push(FILE{name: "abs", size: 123123});
        }
    }
    return (g, base_node);
}

fn part_one<'a>() -> Option<u32>{
    let input: &'a String = &advent_of_code::read_file("inputs", 7);
    let (g, b) = get_graph(&input);
    let (a, b) = get_sum_dirs(&g, b);
    println!("{:?}", b);
    Some(b)
}


fn main() {
    //let input = advent_of_code::read_file("inputs", 7);
    part_one();

    // Create an undirected graph with `i32` nodes and edges with `()` associated data.
    let mut g = Graph::<(&'static str, u32), ()>::new();

    let node1 = g.add_node(("/", 3));
    let node2 = g.add_node(("abc", 123));
    let conn1 = g.add_edge(node1, node2, ());
    // Output the tree to `graphviz` `DOT` format
    for node in g.neighbors_directed(node1,
                                     Direction::Outgoing){
        println!("{:?}", g[node].0);
    }
    for node in g.neighbors_directed(node2,
                                     Direction::Incoming){
        println!("{:?}", g[node].0);
    }
}

/*
struct FILE{
    name: String,
    size: u32,
}

struct DIRECTORY{
    name: String,
    child_dirs: Vec<Rc<DIRECTORY>>,
    parent: Option<Rc<DIRECTORY>>,
    files: Vec<FILE>
}

impl DIRECTORY {
    fn new(name: String) -> Rc<DIRECTORY> {
        Rc::new(DIRECTORY{name, child_dirs: vec![], parent:None, files: vec![]})
    }
    fn add_child_dir(&mut self, child_dir: Rc<DIRECTORY>){
        self.child_dirs.push(child_dir);
        //child_dir.parent = Box(self);
    }
    fn add_file(&mut self, file: FILE){
        self.files.push(file);
    }
}

fn main(){
    let mut dir = DIRECTORY::new(String::from("abc"));
    let mut dir2 = DIRECTORY::new(String::from("abc2"));
    dir.add_child_dir(dir2);
    let mut dir3 = DIRECTORY::new(String::from("abc3"));
    dir2.add_child_dir(dir3);
    println!("{:?}", dir.name);
    println!("{:?}", dir.child_dirs[0].name);
    println!("{:?}", dir.child_dirs[0].child_dirs[0].name);
}
pub struct FILE {
    name: String,
    size: u32
}

pub struct DIR<'a> {
    name: String,
    subdirs: &'a Option<Vec<DIR<'a>>>,
    //parent_dir: Box<Option<&'a DIR<'a>>>,
    parent_dir: &'a Option<DIR<'a>>,
    files: Option<Vec<&'a FILE>>
}

pub fn find_dir<'a>(start_dir: &'a DIR<'a>, name: String) -> &'a Option<DIR<'a>>{
    if name==".."{
        return start_dir.parent_dir;
    }
    if name=="/"{
        let mut dir = start_dir;
        loop {
            if dir.parent_dir.is_none(){
                return Some(dir);
            }
            dir = dir.parent_dir.unwrap();
        }
    }

    //let subdirs = start_dir.subdirs;
    if start_dir.subdirs.is_some(){
        for dir in start_dir.subdirs.as_ref().unwrap(){
            if dir.name==name {
                return Some(dir);
            }
        }
    }
    //let mut id: Option<usize> = None;
    //for (idx, dir) in start_dir.subdirs.unwrap().iter().enumerate() { //unwrap() {
    //    if dir.name == name {
    //        id = Some(idx);
    //        break;
    //    }
    //}
    //if id.is_some() {
    //    return start_dir.subdirs.unwrap()[id.unwrap()];
    //}
    // new dir
    //
    //start_dir
    //let dir: DIR = DIR{name,
    //                   parent_dir:Box::new(Some(start_dir)),
    //                   subdirs: None,
    //                   files: None};
    //return DIR{name,
    //           parent_dir:Box::new(Some(start_dir)),
    //           subdirs: None,
    //           files: None};
    None
}
pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
//static base_dir: DIR = DIR{name:String::from("/"),
//                           parent_dir:None,
//                           subdirs: None,
//                           files: None};
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);

    /*
    let mut current_dir: DIR = DIR{name:String::from("/"),
                               parent_dir:None,
                               subdirs: None,
                               files: None};
    let mut current_dir2: DIR = DIR{name:String::from("test"),
                                parent_dir:Some(&current_dir),
                                subdirs: None,
                                files: None};

    current_dir.subdirs = Some(vec![&current_dir2]);
    let next_dir: Option<&DIR> = find_dir(&current_dir, String::from("test"));

    //println!("{:?}", current_dir3.parent_dir.unwrap().name);
    //println!("{:?}", current_dir3.parent_dir.unwrap().name);

    println!("{:?}", next_dir.unwrap().name);
    println!("{:?}", next_dir.unwrap().name);

    //let file = FILE{name:String::from("test"), size:1212312};

    //println!("{:?}", vec![file]);
    //let dir  = DIR{name:String::from("test_dir"),
    //               files:Some(vec![file]),
    //               subdirs:None};
    */
    //println!("{:?} {:?}", dir.name, dir.files.unwrap()[0].name);
    //advent_of_code::solve!(1, part_one, input);
    //advent_of_code::solve!(2, part_two, input);
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
