use n_puzzle::solver::resolve::resolve::parse_and_play;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_and_play(args)
    {
        Ok(_) => println!("Grid Solve"),
        Err(_) => println!("Time out"),
    }
}