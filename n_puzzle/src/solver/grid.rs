pub mod grid{  
    use crate::solver::tree::tree::Distance;
    #[derive(Copy, Hash, Eq, PartialEq, Debug, Clone)]
    pub struct Case
    {
        pub number: i32,
        pub pos_x: i32,
        pub pos_y: i32,
    }
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub struct Grid
    {
        pub grid: Vec<Vec<Case>>, 
        pub size: usize, //dimensiom 3x3 4x4
        
    }

    impl Grid
    {
        pub fn new(size_p: usize) -> Self
        {
            Grid{
                size : size_p,
                grid: Vec::with_capacity(size_p as usize),
            }
        }
    
        pub fn full_of_zero(size_p: usize) -> Self
        {
            Grid{
                size: size_p,
                grid: full_of_zero(size_p),
            }
        }

        pub fn print_grid(&self)
        {
            for i in 0..self.size
            {
                print!("|");
                for j in 0..self.size
                {
                    print!("{}", self.grid[i][j].number);
                    if j != self.size - 1{
                        print!(" ");
                    }
                }
                println!("|");
            }
            println!("");
        }

        pub fn print_full_grid(&self)
        {
            for i in 0..self.size
            {
                println!("{:?}", self.grid[i]);
            }
        }


        pub fn distance_number(&self, other: &Grid, ch_distance: usize, vec_dist: &Vec<Distance>) -> u32
        {
            let mut sum_distance: u32 = 0;
            for i in &self.grid
            {
                for j in i{
                    sum_distance += vec_dist[ch_distance](j, other);
                }
            }
            // println!("sum {}", sum_distance);
            sum_distance
        }
        
        
    }

    pub fn fill_distance() -> Vec<for<'r, 's> fn(&'r Case, &'s Grid) -> u32>
    {
        type Distance = fn(&Case, &Grid) -> u32;
        let array_of_fill: Vec<Distance> = vec![manhattan_distance, euclidien_distance, tiles_out_of_place];
        array_of_fill
    }
    
    pub fn full_of_zero(size: usize) -> Vec<Vec<Case>>
    {
        let mut new_grid: Vec<Vec<Case>>;
        let mut new_line: Vec<Case>;
        // let mut new_line: Vec<Case>;
        if size == 0 {panic!("size needs to be a real number different of null")}
        new_grid = vec![];
        new_line = vec![];
        // new_line = Vec::<Case>::with_capacity(1 as usize);
        for j in 0..size{
            new_line.clear();
            for i in 0..size{
                new_line.push(Case{number: 0 as i32, pos_x: i as i32, pos_y: j as i32});
            }
            new_grid.push(new_line.clone());
        }
        
        new_grid
    }

    pub fn init_vec_of_case(vec_u: Vec<Vec<u32>>) -> Vec<Vec<Case>>
    {
        let mut vec: Vec<Vec<Case>> = vec![];
        let mut vec_of_case: Vec<Case>;
        
        for j in 0..vec_u.len()
        {
            vec_of_case = vec![];
            for i in 0..vec_u.len()
            {
                vec_of_case.push(Case{number: vec_u[j][i] as i32, pos_x: i as i32, pos_y: j as i32});
            }
            vec.push(vec_of_case.to_vec());
        }
        // println!("{:?}", vec);
        return vec;
    }

    pub fn abs(a: i32) -> u32{
        if a < 0 {((a * - 1) as u32).try_into().unwrap()}
        else { return a as u32 }
    }

    pub fn manhattan_distance(case: &Case, other: &Grid) -> u32{
        let mut distance= 0;
        for i in &other.grid
        {
            for j in i{
                if j.number - case.number == 0 && j.number != 0{
                   distance = abs((j.pos_x - case.pos_x).try_into().unwrap()) + abs((j.pos_y - case.pos_y).try_into().unwrap());                   
                }
            }
        }
        distance
    }

    pub fn euclidien_distance(case: &Case, other: &Grid) -> u32{
        let mut distance= 0;
        let mut x;
        let mut y;
        for i in &other.grid
        {
            for j in i{
                if j.number - case.number == 0 {
                    x = abs((j.pos_x - case.pos_x).try_into().unwrap());
                    y = abs((j.pos_y - case.pos_y).try_into().unwrap());
                    distance = x * x + y * y;
                }
            }
        }
        distance
    }

    pub fn tiles_out_of_place(case: &Case, other: &Grid) -> u32{
        for i in &other.grid
        {
            for j in i{
                if j.number - case.number == 0 && 
                j.pos_x == case.pos_x && 
                j.pos_y == case.pos_y {
                    return 0;
                }
            }
        }
        return 1
    }
    
}