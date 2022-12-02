use std::fs;
mod solutions;
use solutions::day1::solution as day1_solution;
// use solutions::day2::solution as day2_solution;

fn main() {
    println!("day 1 solution Part 1: {:?},  solution Part 2: {:?}", day1_solution::solve(), day1_solution::solve2());
    // println!("day 1 solution: {:?}", day2_solution::solve());
}
