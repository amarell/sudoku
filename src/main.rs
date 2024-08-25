use core::fmt;
use std::{fmt::Display, result};

type Result<T> = result::Result<T, ()>;

fn printing(g: &Vec<Vec<i32>>) {
    for r in g {
        println!("{:?}", r);
    }
}

fn main() {
    let input = vec![
        vec![3, 0, 6, 5, 0, 8, 4, 0, 0],
        vec![5, 2, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 8, 7, 0, 0, 0, 0, 3, 1],
        vec![0, 0, 3, 0, 1, 0, 0, 8, 0],
        vec![9, 0, 0, 8, 6, 3, 0, 0, 5],
        vec![0, 5, 0, 0, 9, 0, 6, 0, 0],
        vec![1, 3, 0, 0, 0, 0, 2, 5, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 7, 4],
        vec![0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    if let Ok(solution) = solve_naive(&input) {
        printing(&solution)
    } else {
        println!("No solution.")
    }
}

/// Returns solved sudoku grid if the solution exists
/// If solution doesn't exist, returns an error.
fn solve_naive(input: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>> {
    let mut grid = input.clone();

    if f(&mut grid, 0, 0) {
        Ok(grid)
    } else {
        Err(())
    }
}

fn f(g: &mut Vec<Vec<i32>>, r: usize, c: usize) -> bool {
    if (r == 8) && (c == 9) {
        return true;
    }

    if c == 9 {
        return f(g, r + 1, 0);
    }

    if g[r][c] != 0 {
        return f(g, r, c + 1);
    }

    // The spot is not filled, attempt all possiblities
    for i in 1..=9 {
        if is_safe(g, r, c, i) {
            g[r][c] = i;
            if f(g, r, c + 1) {
                return true;
            }

            // Guess was wrong, reset the grid to initial state
            g[r][c] = 0
        }
    }

    // Assumption was wrong somewhere along the way or there is no solution
    false
}

fn solve(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![vec![]]
}

fn is_safe(g: &Vec<Vec<i32>>, r: usize, c: usize, num: i32) -> bool {
    // Check row safety
    for &el in &g[r] {
        if el == num {
            return false;
        }
    }

    // Check column safety
    for r in 0..9 {
        if g[r][c] == num {
            return false;
        }
    }

    // Check box safety
    let box_row = r / 3;
    let box_col = c / 3;

    let cols_to_check = box_col * 3..(box_col * 3 + 3);
    let rows_to_check = box_row * 3..(box_row * 3 + 3);

    for r in rows_to_check.clone() {
        for c in cols_to_check.clone() {
            if g[r][c] == num {
                return false;
            }
        }
    }

    true
}

mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        let input = vec![
            vec![3, 0, 6, 5, 0, 8, 4, 0, 0],
            vec![5, 2, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 8, 7, 0, 0, 0, 0, 3, 1],
            vec![0, 0, 3, 0, 1, 0, 0, 8, 0],
            vec![9, 0, 0, 8, 6, 3, 0, 0, 5],
            vec![0, 5, 0, 0, 9, 0, 6, 0, 0],
            vec![1, 3, 0, 0, 0, 0, 2, 5, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 7, 4],
            vec![0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let expected = vec![
            vec![3, 1, 6, 5, 7, 8, 4, 9, 2],
            vec![5, 2, 9, 1, 3, 4, 7, 6, 8],
            vec![4, 8, 7, 6, 2, 9, 5, 3, 1],
            vec![2, 6, 3, 4, 1, 5, 9, 8, 7],
            vec![9, 7, 4, 8, 6, 3, 1, 2, 5],
            vec![8, 5, 1, 7, 9, 2, 6, 4, 3],
            vec![1, 3, 8, 9, 4, 7, 2, 5, 6],
            vec![6, 9, 2, 3, 5, 1, 8, 7, 4],
            vec![7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert_eq!(expected, solve_naive(&input).unwrap())
    }
}
