use std::fs::File;
use std::io::{self, BufRead};

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

fn search(matrix: &[Vec<char>]) -> usize {
    let num_rows = matrix.len();
    let num_cols = if num_rows > 0 { matrix[0].len() } else { 0 };

    let mut count = 0;
    for r in 0..num_rows {
        for c in 0..num_cols {
            if !(matrix[r][c] == 'A') {
                continue;
            }

            if r + 1 < num_rows && c + 1 < num_cols && r  >= 1 && c >= 1 {
                    if ((matrix[r-1][c-1] == 'S' && matrix[r+1][c+1] == 'M') || (matrix[r-1][c-1] == 'M' && matrix[r+1][c+1] == 'S')) 
                        && ((matrix[r-1][c+1] == 'S' && matrix[r+1][c-1] == 'M') || (matrix[r-1][c+1] == 'M' && matrix[r+1][c-1] == 'S')) {
                            count += 1;
                    }
            }
        }
    }

    return count
}


fn main() {
    let filename = "input.txt";

    match read_input(filename) {
        Ok(matrix) => {
            let count = search(&matrix);
            println!("{}", count);
        }
        Err(e) => {
            eprintln!("reading file: {}", e);
        }
    }
}
