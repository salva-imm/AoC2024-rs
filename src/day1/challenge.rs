use std::fs;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string("./src/day1/input.txt")
        .expect("Something went wrong reading the file");
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];
    let _temp: Vec<_> = contents.lines().map(|line| {
        let splited_line: Vec<i32>  = line.parse::<String>().unwrap().split(" ").map(|num| {
            num.parse::<i32>().unwrap_or(-1)
        }).collect();
        first_list.push(splited_line[0]);
        second_list.push(splited_line[3]);
    }).collect();
    first_list.sort();
    second_list.sort();
    (first_list, second_list)

}


pub fn day1() -> i32 {
    let (first_list, second_list) = read_input();
    let it = first_list.iter().zip(second_list.iter());

    let mut sum = 0;
    for (first, second) in it {
        sum += (first - second).abs();
    }
    sum
}