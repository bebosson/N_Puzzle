## ABOUT THE PROJECT

Welcome to my n_puzzle resolution program
A project for 42 school 

this program was written entirely in rust. To use it you need to run the shell script included in the "scripts" folder to install cargo, and because we need python2 for running the generator of grid, to install python2 as well:
```
# chmod a+x install_cargo_python2 
# ./install_cargo_python2
```
## Usage 

cargo run -- [distance_choosen: 0-manhattan 1-euclidien 2-tiles out of place] [path of the file] [set time out: y-yes, n-no]
Thoses distance are admissible because the estimate distance are always under the true cheapest cost from n to goal

## exemple 
```
cargo run -- 2 ex.txt y
```
## description 

--------------------------------------------------------------------
cargo will run the resolution of the table ex.txt with the euclidien distance as the heuristic function,
and time out if it takes more than 10 seconds

the input file needs to be written as follow:
```
#some comment
3#size of the grid with size > 2
#no dublicate and numbers are include between 0 and size * size - 1
0 2 3
1 4 5
8 7 6
--------------------------------------------------------------------

The algorithm will stop when the time_out or the aim solution are reached
The solution will always be in the the form of the snail
1 2 3
8 0 4
7 6 5
```

you can use the generator of grid n_puzzle-gen.py with this command
```
# python2 scripts/n_puzzle-gen.py -[n: as number with n > 2] -[s: solvable, u: unsolvable] > path_of_your_file 
```
then to solve it:
```
# cargo run -- [0;2] "relative path of your file"
exemple
# cargo run -- 2 ./grid/ex.txt
```
you can automate and test multiple randomize grids with thoses command:
```
# cargo build
====> erase all generated grids in the target folder and generates new one, then compile and create the binary files 
```
```
# cargo test -- [--nocapture: for printing the output in the stdout]
```
====> run the test programs, include in the tests folder (you have static grids located in the bad_grid and good_grid folders)
```
# cargo clean
```
====> erase all generated grids files and binaries

