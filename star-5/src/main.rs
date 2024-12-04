use std::fs;
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let memory: String = fs::read_to_string("input.txt")?;

    let mut mul = Vec::new();

    let pattern = r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)";
    let re = Regex::new(pattern).expect("Invalid regex");

    for (_, [x, y]) in re.captures_iter(&memory).map(|c| c.extract()) {
        let x = x.parse::<i32>()?;
        let y = y.parse::<i32>()?;

        mul.push(x * y);
    }

    let sum: i32 = mul.iter().sum();
    println!("{}", sum);
    Ok(())
}
