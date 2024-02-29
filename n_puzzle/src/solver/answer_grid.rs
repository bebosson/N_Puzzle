pub mod answer_grid{

    use crate::solver::grid::grid::*;
    use crate::solver::tree::tree::Answer;
    // use crate::solver::tree

    pub fn right_fill(answer : &Answer, pos_y: usize, pos_x: usize, move_i: usize, ride_number: usize) -> (usize, usize, usize, usize)
    {
        if pos_x + 1 > answer.grid.size - ride_number {answer.move_solution[move_i + 1](answer, pos_y, pos_x, move_i + 1, ride_number)} else {(pos_y, pos_x + 1, move_i, ride_number)} 
    }
    
    fn left_fill(answer : &Answer, pos_y: usize, pos_x: usize, move_i: usize, ride_number: usize) -> (usize, usize, usize, usize)
    {
        if pos_x < ride_number {answer.move_solution[move_i + 1](answer, pos_y, pos_x, move_i + 1, ride_number + 1)} else {(pos_y, pos_x - 1, move_i, ride_number)}
    }
    
    fn up_fill(answer : &Answer, pos_y: usize, pos_x: usize, move_i: usize, ride_number: usize) -> (usize, usize, usize, usize)
    {
        if pos_y < ride_number {answer.move_solution[0](answer, pos_y, pos_x, 0, ride_number)} else {(pos_y - 1, pos_x, move_i, ride_number)}
    }
    
    fn down_fill(answer : &Answer ,pos_y: usize, pos_x: usize, move_i: usize, ride_number: usize) -> (usize, usize, usize, usize)
    {
        if pos_y + 1 > answer.grid.size - ride_number {answer.move_solution[move_i + 1](answer, pos_y, pos_x, move_i + 1, ride_number)} else {(pos_y + 1, pos_x, move_i, ride_number)}
        
    }
    
    pub fn fill_array() -> Vec<for<'r> fn(&'r Answer, usize, usize, usize, usize) -> (usize, usize, usize, usize)>
    {
        type Fillmove = fn(&Answer, usize, usize, usize, usize) -> (usize, usize, usize, usize);
        let array_of_fill: Vec<Fillmove> = vec![right_fill, down_fill, left_fill, up_fill];
        array_of_fill
    }
    
    impl Answer{
        pub fn snail_solution(&mut self)
        {
            let mut movement : (usize, usize, usize, usize) = (0, 0, 0, 1);    
            for i in 0..self.vec_number.len()
            {
                self.grid.grid[movement.0][movement.1] = Case{ number: self.vec_number[i] as i32, pos_x: movement.1 as i32, pos_y: movement.0 as i32};
                movement = self.move_solution[movement.2](&self, movement.0, movement.1, movement.2, movement.3);
            }
            self.grid.grid[movement.0][movement.1] = Case{ number: 0 as i32, pos_x: movement.1 as i32, pos_y: movement.0 as i32};
        }
    }
    
    pub fn create_solution(size :usize) -> Answer
    {
        let mut solution = Answer::init(size);
        solution.snail_solution();
        solution
    }
}

