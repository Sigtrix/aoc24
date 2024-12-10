use std::fs;
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let memory: String = fs::read_to_string("input.txt")?;

    let mut mul = Vec::new();

    let pattern = r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)|(?<on>do\(\))|(?<off>don't\(\))";
    let re = Regex::new(pattern).expect("Invalid regex");

    let mut enabled = true;
    for cap in re.captures_iter(&memory) {
        if let Some(_on) = cap.name("on") {
            enabled = true;
        } else if let Some(_off) = cap.name("off") {
            enabled = false;
        } else if let (Some(x), Some(y)) = (cap.name("x"), cap.name("y")) {
            let x: i32 = x.as_str().parse()?;
            let y: i32 = y.as_str().parse()?;
            if enabled {
                mul.push(x * y);
            }
        }
    }

    let sum: i32 = mul.iter().sum();
    println!("{}", sum);
    Ok(())
}

