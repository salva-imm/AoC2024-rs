use std::fs;
use itertools::Itertools;
use regex::Regex;

fn read_and_parse_input() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("./src/day3/input.txt")
        .expect("Something went wrong reading the file");
    let mut list: Vec<(i32,i32)> = vec![];
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let muls: Vec<Vec<String>> = contents
        .lines()
        .filter_map(|line| {
            let mut results = Vec::new();
            for caps in re.captures_iter(line) {
                if let Some(matched) = caps.get(0) {
                    let parts: Vec<String> = matched
                        .as_str()
                        .replace("mul(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|s| s.to_string())
                        .collect();
                    results.push(parts);
                }
            }
            if results.is_empty() {
                None
            } else {
                Some(results)
            }
        })
        .flatten()
        .collect();

    muls
}



pub fn day3() -> i32{
    let parsed_data = read_and_parse_input();
    let mut sum = 0;
    for data in &parsed_data {
        sum += data[0].parse::<i32>().unwrap() * data[1].parse::<i32>().unwrap();
    }
    sum
}
