use std::fs::File;
use std::io::{self, BufRead};

const XMAS: &str = "XMAS";

fn read_input(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let mut matrix = Vec::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }
    Ok(matrix)
}

fn search(matrix: &[Vec<char>], target: &str) -> usize {
    let target_chars: Vec<char> = target.chars().collect();
    let target_len = target_chars.len();
    let num_rows = matrix.len();
    let num_cols = if num_rows > 0 { matrix[0].len() } else { 0 };

    let mut count = 0;
    for r in 0..num_rows {
        for c in 0..num_cols {
            if c + target_len <= num_cols {
                if (0..target_len).all(|i| matrix[r][c + i] == target_chars[i]) {
                    count += 1;
                }
            }
            if r + target_len <= num_rows {
                if (0..target_len).all(|i| matrix[r + i][c] == target_chars[i]) {
                    count += 1;
                }
            }
            if r + target_len <= num_rows && c + target_len <= num_cols {
                if (0..target_len).all(|i| matrix[r + i][c + i] == target_chars[i]) {
                    count += 1;
                }
            }
            if r + target_len <= num_rows && c >= target_len - 1 {
                if (0..target_len).all(|i| matrix[r + i][c - i] == target_chars[i]) {
                    count += 1;
                }
            }
        }
    }

    return count
}


fn main() {
    let filename = "input.txt";
    let target = XMAS;
    let rev_target: String = XMAS.chars().rev().collect();

    match read_input(filename) {
        Ok(matrix) => {
            let mut count = search(&matrix, target);
            count += search(&matrix, &rev_target);
            println!("{}", count);
        }
        Err(e) => {
            eprintln!("reading file: {}", e);
        }
    }
}
