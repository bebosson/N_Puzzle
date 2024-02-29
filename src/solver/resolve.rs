pub mod resolve{
    use crate::solver::tree::tree::{Tree, Node, Answer, Distance};
    use std::time::{Instant, Duration};
    use crate::solver::parser::parser::parser;
    impl Node{

        pub fn create_reset_and_push(& mut self, answer: &Answer, dist: usize, vec_dist: &Vec<Distance>, tr_back: u32, swap: fn(&mut Node))
        {
            let mut node = self.clone();
            swap(& mut node);
            node.list_swap.push(tr_back);
            self.reset_and_push(node, answer, dist, vec_dist);
        }

        pub fn reset_and_push(& mut self, mut node: Node, answer: &Answer, ch_distance: usize, vec_dist: &Vec<Distance>)
        {
            node.next_move.clear();
            node.costsofar = self.costsofar + 1;
            node.distance = node.grid.distance_number(&answer.grid, ch_distance, vec_dist);
            node.sum = node.costsofar + node.distance;
            node.swap_possible();
            self.next_move.push(node);
        }

        pub fn push_front_into_resolve(&self, vec_resolve: & mut Vec<Node>)
        {
            for i in &self.next_move
            {
                vec_resolve.push(i.clone());
            }
        }
    }

    impl Tree{

        pub fn push_hashset(& mut self, node: & mut Node)
        {
            let clone = node.next_move.clone();
            let mut i = 0;
            while i < node.next_move.len()
            {
                if self.hash.insert(clone[i].grid.clone()) == false
                {
                    node.next_move.remove(i);
                }
                i += 1;
            }
        }

        pub fn compare_distance_nodes(& mut self)
        {
            let mut distance_min = self.distance_min;
            for i in &self.resolve
            {
                if distance_min > i.distance {distance_min = i.distance;}
            }
            self.distance_min = distance_min;
        }

        pub fn check_node_push_open_list(& mut self, node: & mut Node)
        {
            let mut node_clone = node.clone(); 
            self.push_hashset(& mut node_clone);
            node_clone.push_front_into_resolve(&mut self.resolve);
            self.resolve.sort_by(|a , b| b.sum.partial_cmp(&a.sum).unwrap());
            self.compare_distance_nodes();
        }

        pub fn play(& mut self)
        {
            let mut node = self.resolve.pop().unwrap();
            self.complex_time += 1;
            node.swap_possible();
            node.swap_case(&self.answer_grid, self.ch_distance, &self.vec_distance);
            self.check_node_push_open_list(& mut node.clone());
            self.resolve.sort_by(|a , b| b.sum.partial_cmp(&a.sum).unwrap());
                
        }

        pub fn print_stats(&self, time: &Duration){
            println!("time : {} distance_min {} open_list {} node_visited {}", time.as_secs(), self.distance_min, self.complex_size, self.complex_time + 1);
        }

        pub fn print_openlist(&self)
        {
            for i in self.resolve.clone()
            {
                i.print_node();
            }
        }
    }
   
    pub fn parse_and_play(args: Vec<String>) ->  Result<String, &'static str>
    {
        let now = Instant::now();
        let mut tree: Tree;
        let mut elapsed_time = now.elapsed();
        tree = parser(args).unwrap();
        
        while tree.distance_min > 0
        {
            tree.play();
            elapsed_time = now.elapsed();
            if elapsed_time.as_secs() > tree.resolve_time
            {
                tree.complex_size = tree.resolve.len() as u32;
                tree.print_stats(&elapsed_time);
                return Err("time out");
            }   
        }
        if tree.distance_min == 0
        {
            tree.complex_size = tree.resolve.len() as u32;
            tree.traceback();
            tree.print_stats(&elapsed_time);
        }
        Ok("Grid solve".to_string())
    }

}