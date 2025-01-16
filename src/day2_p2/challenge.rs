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

fn detector(data: Vec<i32>) -> bool {
    data.clone().into_iter().tuple_windows().all(|(a, b)| { a <= b && 0 < (a - b).abs() && (a - b).abs() < 4 })||
        data.into_iter().tuple_windows().all(|(a, b)| { b <= a && 0 < (a - b).abs() && (a - b).abs() < 4 })
}


pub fn day2_p2() -> i32{
    let list: Vec<Vec<i32>> = read_input();

    let list: Vec<Vec<i32>> = read_input();
    let mut safe_datas = 0;
    for mut line in list{
        if detector(line.clone()){ // safe!
            safe_datas += 1;
        }else{
            for i in 0..line.len(){

            let mut temp_line = line.clone();
            temp_line.remove(i);
            if detector(temp_line){ // safe!
                safe_datas += 1;
                break;
            }
            }

        }
    }
    safe_datas
}
