pub mod parser{
pub type Parserfn = fn(& mut ParserStats, u32);
use core::panic;
use std::io::*;
use std::fs::*;
use std::usize;
use crate::solver::grid::grid::*;
use crate::solver::tree::tree::{init_tree};
use crate::solver::tree::tree::Tree;
use std::num::*;
use std::result::Result;


// pub comment
pub struct ParserStats
{
    size: i32,
    count_ibl: u32,
    count_line: i32,
    grid_parse: Option<Grid>,
    vec_number: Option<Vec<u32>>,
    state_parser: usize,
    fn_parser: Vec<Parserfn>,
}

impl ParserStats
{
    pub fn new() -> Self
    {
        ParserStats{
            size: -1,
            count_ibl: 0,
            count_line: 0,
            grid_parse: None,
            vec_number: None,
            state_parser: 0,
            fn_parser: fill_parser_fn(), 
        }
    }

    pub fn half_parse(p_size: u32) -> Self
    {
        ParserStats { 
            size: -1, 
            count_ibl: 0, 
            count_line: 0, 
            grid_parse: Some(Grid::full_of_zero(p_size as usize)), 
            vec_number: Some((0..(p_size * p_size)).collect()), 
            state_parser: 1, 
            fn_parser: fill_parser_fn() 
        }
    }

    pub fn error_count_ibl(&self) -> Option<bool>
    {
        match self.size as u32 >= self.count_ibl
        {
            true => Some(true),
            false => None,
        }
    }

    pub fn error_count_line(&self) -> Result<bool, &str>
    {
        match self.size == self.count_line
        {
            true => Ok(true),
            false => Err("grid has wrong size or not well formatted"),
        }
    }

    pub fn check_and_remove(& mut self, number: u32) -> Option<bool>
    {
        let mut index: usize = 0;
        let vec_cpy = self.vec_number.clone().unwrap();
        for i in vec_cpy
        {
            if number == i
            {
                self.vec_number.as_mut().unwrap().remove(index);
                return Some(true);
            }
            index += 1;
        }
        None
    }

    pub fn look_for_int(& mut self, line: &String)
    {
        for str in line.split_ascii_whitespace()
        {
            match str.parse::<u32>()
            {
                Ok(i) => {
                    self.fn_parser[self.state_parser](self, i);
                 }
                Err(i) => match i.kind() {
                    IntErrorKind::Empty => panic!("case empty"),
                    IntErrorKind::InvalidDigit => panic!("Invalid Input"),
                    _ => panic!("else"),
                } 
            }
        }
        self.count_ibl = 0;
    }

    pub fn sub_line(& mut self, line: &String)
    {
        let pos_comment: Option<usize> = line.chars().position(|c| c == '#');
        let pos_eol: Option<usize> = line.chars().position(|c| c == '\n');
        match pos_comment
        {
            Some(pos) => {
                if  pos == 0 { return ;}
                else    {self.look_for_int(&line[0..pos].to_string())}
            }
            None => {
                if let Some(pos) = pos_eol { self.look_for_int(&line[0..pos].to_string()) }
            }
        }
    }

    pub fn parse_file(&mut self, path_file: &String)
    {
        let file = read_file(path_file);
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let mut f = 1;
        while f > 0
        {
            f = reader.read_line(&mut buffer)
            .expect("reading won't fail");
            if f == 0 { break ;};
            self.sub_line(&buffer);
            buffer.clear();
        }
        self.error_count_line().expect("bad parsing");
    }
}


    pub fn fill_parser_fn() -> Vec<for<'r> fn(&'r mut ParserStats, u32)>
    {
        pub type Parserfn = fn(& mut ParserStats, u32);
        let array_of_fill: Vec<Parserfn> = vec![count, save_into_grid];
        array_of_fill
    }

    pub fn count(parser: &mut ParserStats, i: u32)
    {
        if parser.size < 0 {
            if i >= 3{ parser.size = i as i32 } else { panic!("size too little"); }
        }
        else {
            parser.count_ibl += 1;
            if parser.count_ibl > parser.size as u32 - 1 {parser.count_line += 1}
        }
        parser.error_count_ibl().unwrap();
    }

    pub fn save_into_grid(parser: & mut ParserStats, i: u32)
    {
        let grid;
        if parser.size > 0 
        {
            parser.check_and_remove(i).expect("duplicate or out of range input"); 
            grid = parser.grid_parse.as_mut().unwrap();
            grid.grid[parser.count_line as usize][parser.count_ibl as usize].number = i as i32;
            parser.count_ibl += 1;
            if parser.count_ibl == parser.size as u32 {parser.count_line += 1; parser.count_ibl = 0;}
        }
        else { parser.size = i as i32;}
    }

    pub fn read_file(path: &String) -> File
    {
        let w = match File::open(path)
        {
            Ok(w) => w,
            Err(_e) => {
                panic!("file doesn't exist");
                }, 
        };
        w
    }

    pub fn check_vec_option(vec_option: Vec<String>) -> (usize, String, bool)
    {
        let heuristic_err;
        if vec_option.len() != 4
        {
            panic!("option needs to be well written and well factored");
        }
        else 
        {
            heuristic_err = vec_option[1].parse::<usize>().unwrap();
            if heuristic_err > 2
            {
                panic!("heuristic needs to be in a range of [0;2]");
            }
        }
        (heuristic_err, vec_option[2].to_string(), check_time_out(vec_option[3].to_string()))
    }

    pub fn check_time_out(arg_time: String) -> bool
    {
        if !arg_time.eq(&"y".to_string()) && !arg_time.eq(&"n".to_string())
        {
            panic!("time_out option needs to be y or n")
        }
        return  arg_time.eq(&"y".to_string());
    }

    

    pub fn parser(vec_option: Vec<String>) -> Result<Tree, &'static str>
    {
        let mut parser = ParserStats::new();
        let heuristic;
        let path;
        let tree_parse;
        let time_out: bool;

        (heuristic, path, time_out) = check_vec_option(vec_option);
        parser.parse_file(&path);
        parser = ParserStats::half_parse(parser.size as u32);
        parser.parse_file(&path);
        println!("time_out: {}", time_out);
        tree_parse = init_tree(parser.size as usize, parser.grid_parse.unwrap(), heuristic, time_out);
        Ok(tree_parse)
    }

}