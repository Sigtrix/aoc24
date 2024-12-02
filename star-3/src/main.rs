use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Could not parse input");
        
        let sign = report[0] - report[1];
        if sign < 0 {
            let mut safe = true;
            for i in 0..report.len() - 1 {
                if report[i] - report[i+1] > -1 || report[i] - report[i+1] < -3 {
                    safe = false;
                }
            }
            
            if safe {
                count += 1;
            }
        } else {
            let mut safe = true;
            for i in 0..report.len() - 1 {
                if report[i] - report[i+1] < 1 || report[i] - report[i+1] > 3 {
                    safe = false;
                }
            }
            
            if safe {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
