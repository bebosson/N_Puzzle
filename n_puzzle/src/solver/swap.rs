pub mod swap{
    use crate::solver::tree::tree::{Node, Answer, Distance};
    impl Node
    {
        pub fn swap_possible(& mut self)
        {
            self.swap_possible = [false; 4];
            let size: i32 = self.grid.size as i32;

            if self.coor_case_void.0 >= 1 { self.swap_possible[0] = true; } // left
            if self.coor_case_void.0 + 1 < size.try_into().unwrap() { self.swap_possible[1] = true; } // right
            if self.coor_case_void.1 >= 1  { self.swap_possible[2] = true; } // up
            if self.coor_case_void.1 + 1 < size.try_into().unwrap() { self.swap_possible[3] = true ; } //down
        }

        pub fn swap_case(& mut self, answer: &Answer, dist: usize, vec_dist: &Vec<Distance>)
        {
            if self.swap_possible[0] { self.create_reset_and_push(answer, dist, vec_dist, 0, Node::swap_left); } //down
            if self.swap_possible[1] { self.create_reset_and_push(answer, dist, vec_dist, 1, Node::swap_right); } // right
            if self.swap_possible[2] { self.create_reset_and_push(answer, dist, vec_dist, 2, Node::swap_up); } // up
            if self.swap_possible[3] { self.create_reset_and_push(answer, dist, vec_dist, 3, Node::swap_down); } //down
        }

        pub fn swap_left(node: & mut Node)
        {
            node.grid.grid[node.coor_case_void.1][node.coor_case_void.0].number = node.grid.grid[node.coor_case_void.1][node.coor_case_void.0 - 1].number;
            node.grid.grid[node.coor_case_void.1][node.coor_case_void.0 - 1].number = 0;
            node.coor_case_void = (node.coor_case_void.0 - 1, node.coor_case_void.1);
        }

        pub fn swap_right(& mut self)
        {
            self.grid.grid[self.coor_case_void.1][self.coor_case_void.0].number = self.grid.grid[self.coor_case_void.1][self.coor_case_void.0 + 1].number;
            self.grid.grid[self.coor_case_void.1][self.coor_case_void.0 + 1].number = 0;
            self.coor_case_void = (self.coor_case_void.0 + 1, self.coor_case_void.1);
        }

        pub fn swap_down(& mut self)
        {
            self.grid.grid[self.coor_case_void.1][self.coor_case_void.0].number = self.grid.grid[self.coor_case_void.1 + 1][self.coor_case_void.0].number;
            self.grid.grid[self.coor_case_void.1 + 1][self.coor_case_void.0].number = 0;
            self.coor_case_void = (self.coor_case_void.0, self.coor_case_void.1 + 1);
        }

        pub fn swap_up(& mut self)
        {
            self.grid.grid[self.coor_case_void.1][self.coor_case_void.0].number = self.grid.grid[self.coor_case_void.1 - 1][self.coor_case_void.0].number;
            self.grid.grid[self.coor_case_void.1 - 1][self.coor_case_void.0].number = 0;
            self.coor_case_void = (self.coor_case_void.0, self.coor_case_void.1 - 1);
        }
    }
}