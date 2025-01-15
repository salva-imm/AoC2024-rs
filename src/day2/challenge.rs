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


fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator + Clone,
    I::Item: Ord + Clone,
{
    data.clone().into_iter().tuple_windows().all(|(a, b)| a <= b) ||
        data.into_iter().tuple_windows().all(|(a, b)| b <= a)
}


pub fn day2() -> i32{
    let list: Vec<Vec<i32>> = read_input();
    let mut safe_datas = 0;
    for line in list{
        if !is_sorted(&line){
            continue;
        }
        let mut is_unsafe = false;
        for i in 0..line.len()-1{
            if ![3,2,1].contains(&(line[i] - line[i+1]).abs())  {
                is_unsafe = true;
                break;
            }
        }
        if !is_unsafe{
            safe_datas += 1;
        }
    }
    safe_datas
}