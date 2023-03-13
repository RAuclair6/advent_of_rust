use std::env;
use std::fs;

fn main() {
    let mut path = env::current_dir().expect("missing input.txt file in day directory");
    path.push("src/bin/2022_01/input.txt");
    let data = fs::read_to_string(path).expect("Cannot read the file");
    let mut reduced_data: Vec<i32> = data
        .split("\n\n")
        .map(|line| line.split("\n").map(|num| num.parse::<i32>().unwrap()))
        .map(|elf| elf.sum())
        .collect();
    println!("pt 1: {}", reduced_data.iter().max().unwrap());
    reduced_data.sort_by(|a, b| b.cmp(a));
    println!("pt 2: {}", reduced_data[0..3].into_iter().sum::<i32>())
}
