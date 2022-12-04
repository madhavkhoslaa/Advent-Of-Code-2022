use std::fs;
mod solutions;
use solutions::day1::solution as day1_solution;
use solutions::day2::solution as day2_solution;
use solutions::day3::solution as day3_solution;
use solutions::day4::solution as day4_solution;

fn main() {
    println!(
        "day 1 solution Part 1: {:?},  solution Part 2: {:?}",
        day1_solution::solve(),
        day1_solution::solve2()
    );
    println!(
        "day 2 solution: {:?}, solution Part 2: {:?}",
        day2_solution::solve1(),
        day2_solution::solve2()
    );
    println!(
        "day 3 solution Part 1: {:?},  solution Part 2: TODO",
        day3_solution::solve1() // day3_solution::solve2()
    );
    println!(
        "Day4 part 1: {}, Day4 part 2: {}",
        day4_solution::solve1(),
        day4_solution::solve2()
    );
}
