// Day 1: Historian Hysteria

mod day1;
mod day2;
mod day2_p2;
mod day2_better_solution_1;
mod day3;

fn main() {
    println!("Day1: {}", day1::challenge::day1());
    println!("Day2: {}", day2::challenge::day2());
    println!("Day2 better solution 1: {}", day2_better_solution_1::challenge::day2());
    println!("Day2 Part 2: {}", day2_p2::challenge::day2_p2());
    println!("Day3: {}", day3::challenge::day3());
}
