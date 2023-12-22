use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};

fn build_matrix(file: &str) -> DMatrix<char> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut nrows = 0;
    let mut data = Vec::new();
    for line in lines.by_ref() {
        nrows += 1;
        data.extend(line.unwrap().chars());
    }
    data = data.iter().map(|c| if *c == 'S' { 'O' } else { *c }).collect();
    DMatrix::from_row_slice(nrows, data.len() / nrows, &data)
}

fn solution(file: &str, steps: usize) -> usize {
    let mut matrix = build_matrix(file);
    for _ in 0..steps {
        let mut new_matrix = matrix.clone();
        for row in 0..matrix.nrows() {
            for col in 0..matrix.ncols() {
                if matrix[(row, col)] == 'O' {
                    if row > 0 && matrix[(row - 1, col)] == '.' {
                        new_matrix[(row - 1, col)] = 'O';
                    }
                    if row < matrix.nrows() - 1 && matrix[(row + 1, col)] == '.' {
                        new_matrix[(row + 1, col)] = 'O';
                    }
                    if col > 0 && matrix[(row, col - 1)] == '.' {
                        new_matrix[(row, col - 1)] = 'O';
                    }
                    if col < matrix.ncols() - 1 && matrix[(row, col + 1)] == '.' {
                        new_matrix[(row, col + 1)] = 'O';
                    }
                    new_matrix[(row, col)] = '.';
                }
            }
        }
        matrix = new_matrix;
    }
    matrix.iter().filter(|c| **c == 'O').count()
}

fn main() {
    assert_eq!(solution("example.txt", 6), 16);
    assert_eq!(solution("input.txt", 64), 3689);
}
