use std::fs;
use itertools::Itertools;

fn read_input() -> Vec<Vec<i32>> {
    let contents = fs::read_to_string("./src/day2/input.txt")
        .expect("Something went wrong reading the file");
    let mut list: Vec<Vec<i32>> = vec![];
    let _temp: Vec<_> = contents.lines().map(|line| {
        let splited_line: Vec<i32>  = line.split(" ").map(|num| {
            num.parse::<i32>().unwrap()
        }).collect();
        list.push(splited_line);
    }).collect();
    list
}


pub fn day2() -> i32{
    let list: Vec<Vec<i32>> = read_input();
    let mut safe_datas = 0;
    for mut line in list{
        let beginning_len = line.len();
        let mut is_unsafe = false;
        let mut index = 0;
        let mut increasing = 0;
        let mut decreasing = 0;
        while index < line.len() - 1{
            let diff = (line[index] - line[index+1]).abs();
            if diff < 4 && diff > 0{
                if line[index] > line[index+1]{
                    decreasing += 1;
                }else{
                    increasing += 1;
                }
                index += 1;
                continue;
            }else{
                line.remove(index);

            }
        }
        if line.len() == beginning_len && (increasing == 0 || decreasing == 0){ // safe!
            safe_datas += 1;
        }
    }
    safe_datas
}