use std::collections::HashMap;
use std::env;
use std::fs;

fn pt2<'a>(line: (&'a str, &'a str)) -> (&'a str, &'a str) {
    let second = match line.1 {
        "X" => match line.0 {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            _ => "oops",
        },
        "Y" => match line.0 {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            _ => "oops",
        },
        "Z" => match line.0 {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            _ => "oops",
        },
        _ => "oops",
    };
    return (line.0, second);
}

fn main() {
    let mut path = env::current_dir().expect("missing input.txt file in day directory");
    path.push("src/bin/2022_02/input.txt");
    let data = fs::read_to_string(path).expect("Cannot read the file");
    let lines: Vec<(&str, &str)> = data
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .collect();

    let games: HashMap<(&str, &str), i32> = HashMap::from([
        (("A", "X"), 4),
        (("B", "X"), 1),
        (("C", "X"), 7),
        (("A", "Y"), 8),
        (("B", "Y"), 5),
        (("C", "Y"), 2),
        (("A", "Z"), 3),
        (("B", "Z"), 9),
        (("C", "Z"), 6),
    ]);
    println!(
        "pt 1: {}",
        lines
            .iter()
            .map(|line| games.get(line).unwrap())
            .sum::<i32>()
    );
    println!(
        "pt 2: {}",
        lines
            .iter()
            .map(|line| games.get(&pt2(*line)).unwrap())
            .sum::<i32>()
    )
}
