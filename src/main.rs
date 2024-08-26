use std::result;

type Result<T> = result::Result<T, ()>;
type Grid<T> = Vec<Vec<T>>;

fn printing(g: &Grid<i32>) {
    for r in g {
        println!("{:?}", r);
    }
}

fn main() {
    let input = vec![
        vec![0, 0, 0, 0, 0, 2, 0, 0, 0],
        vec![7, 3, 0, 0, 5, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 5, 3, 0],
        vec![5, 0, 0, 0, 4, 0, 0, 0, 0],
        vec![3, 4, 2, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 8, 6, 0, 0, 5, 0],
        vec![9, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 4, 3, 0, 0, 0, 6],
        vec![0, 0, 0, 0, 0, 0, 8, 0, 0],
    ];

    if let Ok(solution) = solve(&input) {
        printing(&solution)
    } else {
        println!("No solution.")
    }
}

/// Returns solved sudoku grid if the solution exists
/// If solution doesn't exist, returns an error.
fn solve(input: &Grid<i32>) -> Result<Grid<i32>> {
    let mut grid = input.clone();

    if solve_sudoku(&mut grid, 0, 0) {
        Ok(grid)
    } else {
        Err(())
    }
}

fn solve_sudoku(g: &mut Grid<i32>, r: usize, c: usize) -> bool {
    if (r == 8) && (c == 9) {
        return true;
    }

    if c == 9 {
        return solve_sudoku(g, r + 1, 0);
    }

    if g[r][c] != 0 {
        return solve_sudoku(g, r, c + 1);
    }

    // The spot is not filled, attempt all possiblities
    for i in 1..=9 {
        if is_safe(g, r, c, i) {
            g[r][c] = i;
            if solve_sudoku(g, r, c + 1) {
                return true;
            }

            // Guess was wrong, reset the grid to initial state
            g[r][c] = 0
        }
    }

    // Assumption was wrong somewhere along the way or there is no solution
    false
}

fn is_safe(g: &Grid<i32>, r: usize, c: usize, num: i32) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
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

        assert_eq!(expected, solve(&input).unwrap())
    }

    #[test]
    fn test_expert_difficulty() {
        let input = vec![
            vec![0, 0, 0, 0, 0, 2, 0, 0, 0],
            vec![7, 3, 0, 0, 5, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 5, 3, 0],
            vec![5, 0, 0, 0, 4, 0, 0, 0, 0],
            vec![3, 4, 2, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 8, 6, 0, 0, 5, 0],
            vec![9, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 4, 3, 0, 0, 0, 6],
            vec![0, 0, 0, 0, 0, 0, 8, 0, 0],
        ];

        let expected = vec![
            vec![6, 8, 5, 3, 1, 2, 4, 7, 9],
            vec![7, 3, 4, 9, 5, 8, 1, 6, 2],
            vec![2, 1, 9, 6, 7, 4, 5, 3, 8],
            vec![5, 6, 8, 2, 4, 7, 9, 1, 3],
            vec![3, 4, 2, 1, 9, 5, 6, 8, 7],
            vec![1, 9, 7, 8, 6, 3, 2, 5, 4],
            vec![9, 2, 6, 7, 8, 1, 3, 4, 5],
            vec![8, 5, 1, 4, 3, 9, 7, 2, 6],
            vec![4, 7, 3, 5, 2, 6, 8, 9, 1],
        ];

        assert_eq!(expected, solve(&input).unwrap())
    }

    #[test]
    fn test_no_solution() {
        let input = vec![
            //   v - This column has two 7s
            vec![7, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![7, 3, 0, 0, 5, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 5, 3, 0],
            vec![5, 0, 0, 0, 4, 0, 0, 0, 0],
            vec![3, 4, 2, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 8, 6, 0, 0, 5, 0],
            vec![9, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 4, 3, 0, 0, 0, 6],
            vec![0, 0, 0, 0, 0, 0, 8, 0, 0],
        ];

        assert_eq!(Err(()), solve(&input))
    }
}
