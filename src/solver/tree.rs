
pub mod tree{
    pub type Distance = fn(&Case, &Grid) -> u32;
    type Fillmove = fn(&Answer, usize, usize, usize, usize) -> (usize, usize, usize, usize);
    use crate::solver::grid::grid::*;
    use crate::solver::answer_grid::answer_grid::{fill_array, create_solution};
    use core::panic;
    use std::collections::HashSet;


    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Node{
        pub grid: Grid,
        pub coor_case_void: (usize, usize),
        pub distance: u32,
        pub swap_possible: [bool;4],
        pub next_move: Vec<Node>,
        pub costsofar: u32,
        pub sum: u32,
        pub list_swap: Vec<u32>,
    }

    pub struct Answer{
        pub grid: Grid,
        pub move_solution: Vec<Fillmove>,
        pub vec_number: Vec<u32>,
    }

    pub struct Tree{
        pub answer_grid: Answer,
        pub depth_max: u32,
        pub distance_min: u32,
        pub resolve: Vec<Node>,
        pub size: usize,
        pub first_node: Node,
        pub hash: HashSet<Grid>,
        pub complex_time: u32,
        pub complex_size: u32,
        pub vec_distance: Vec<Distance>,
        pub ch_distance: usize,
        pub resolve_time: u64,
    }

    impl Node{
        pub fn new(size_p: usize) -> Self
        {
            Node{
                grid: Grid::new(size_p), 
                coor_case_void : (0,0),
                distance: 0,
                swap_possible: [false;4],
                next_move: vec![],
                costsofar: 0,
                sum: 0,
                list_swap: vec![], 
            }
        }

        pub fn from(origine: &Node) -> Self
        {
            Node{
                grid: origine.grid.clone(),
                coor_case_void : (0,0),
                distance: 0,
                swap_possible: [false;4],
                next_move: vec![],
                costsofar: 0,
                sum: 0,
                list_swap: vec![],
            }
        }

        pub fn print_vec_node(&self)
        {
            let mut nbr_node : usize = 0;
            for i in &self.next_move
            {
                println!("node numero : {}", nbr_node);
                i.print_node();
                nbr_node += 1;
            }
        }

        pub fn print_node(&self)
        {
            self.grid.print_grid();
            println!("{:?}", self.coor_case_void);
            println!("{:?}",self.swap_possible);
            println!("---------");
            println!("distance {}", self.distance);
            println!("costsofar {:?}", self.costsofar);
            println!("sum {:?}", self.sum);
            println!("list {:?}", self.list_swap);
            println!("---------");
        }
        pub fn print_all_nextmove(&self)
        {
            println!("---all_next_move---");
            for i in &self.next_move
            {
                i.print_node();
            }
        }
    }

    impl Answer{
        pub fn init(size_p: usize) -> Self
        {
            Answer{
                grid: Grid::full_of_zero(size_p),
                move_solution: fill_array(),
                vec_number: (1..(size_p as u32 * size_p as u32)).collect(),
            }
        }
    }

    impl Tree{
        pub fn new(size_p: usize) -> Self
        {
            Tree{
                answer_grid: create_solution(size_p),
                depth_max: 0,
                distance_min: 0,
                resolve: vec![],
                size : size_p,
                first_node: Node::new(size_p),
                hash: HashSet::<Grid>::new(),
                complex_time: 0,
                complex_size: 0,
                vec_distance: fill_distance(),
                ch_distance: 1,
                resolve_time: 20, //10 * size_p as u64
            }
        }

        pub fn print_tree(&self)
        {
            println!("Tree");
            self.first_node.print_node();
            println!("depth_max {}, distance_min {} size{}", self.depth_max, self.distance_min, self.size);
            println!("distance_node {}", self.first_node.distance);
            println!("complex_size {:?}", self.complex_size);
            println!("complex_time {:?}", self.complex_time);
        }

        pub fn traceback(& mut self)
        {
            let mut moves = 1;
            
            if self.distance_min == 0
            {
               if let Some(x) = self.resolve.last()
               {
                    let mut last_node = x.clone();
                    for i in last_node.list_swap.clone().into_iter().rev()
                    {
                        traceback_swap(i, & mut last_node);
                    }
                    println!("--------START--------"); 
                    last_node.grid.print_grid();
                    for i in last_node.list_swap.clone().into_iter()
                    {
                        print!("---MOVE {} ", moves);
                        all_moves(i, & mut last_node);
                        last_node.grid.print_grid();
                        moves += 1;
                    }
                }
            }
        }
        
    }

    pub fn init_tree(size_p: usize, grid_parse: Grid, heurist: usize, set_time: bool) -> Tree
    {
        let mut node_parse = init_node(size_p, grid_parse);
        let mut tree_foo: Tree = Tree::new(size_p);
        node_parse.distance = node_parse.grid.distance_number(&tree_foo.answer_grid.grid, tree_foo.ch_distance, &tree_foo.vec_distance);
        node_parse.print_node();
        tree_foo.first_node = node_parse.clone();
        tree_foo.distance_min = tree_foo.first_node.distance;
        tree_foo.hash.insert(node_parse.grid);
        tree_foo.resolve.push(tree_foo.first_node.clone());
        tree_foo.ch_distance = heurist;
        if set_time {tree_foo.resolve_time = 10;} else {tree_foo.resolve_time = 100000000}
        tree_foo
    }

    pub fn init_node(size: usize, grid_parse: Grid) -> Node
    {
        let mut node: Node = Node::new(size);
        node.grid = grid_parse.clone();
        node.coor_case_void = look_for_void_case(grid_parse);
        node
    }

    pub fn look_for_void_case(grid_parse: Grid) -> (usize, usize)
    {
        for i in grid_parse.grid
        {
            for j in i
            {
                if j.number == 0
                {
                    return (j.pos_x as usize, j.pos_y as usize);
                }
            }
        }
        panic!("what ? no void case in the grid?")
    }
    pub fn traceback_swap(swap: u32, node: & mut Node)
    {
        if swap == 0 { Node::swap_right(node); }
        if swap == 1 { Node::swap_left(node); }
        if swap == 2 { Node::swap_down(node); }
        if swap == 3 { Node::swap_up(node); }
    }
    pub fn all_moves(swap: u32, node: & mut Node)
    {
        if swap == 0 { println!("left---\n"); Node::swap_left(node); }
        if swap == 1 { println!("right---\n"); Node::swap_right(node); }
        if swap == 2 { println!("up---\n"); Node::swap_up(node); }
        if swap == 3 { println!("down---\n"); Node::swap_down(node); }
    }
}