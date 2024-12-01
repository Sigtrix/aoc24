use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Could not parse input");

        if parts.len() == 2 {
            column1.push(parts[0]);
            column2.push(parts[1]);
        } else {
            println!("Expected two location IDs but got: {}", line);
        }
    }

    column1.sort();
    column2.sort();

    let list_diff: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("{}", list_diff);
}
