use std::{collections::HashMap, result};

type Result<T> = result::Result<T, ()>;
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

    solve_naive(input);
}

fn solve_naive(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    /// There is
    ///  - 9 boxes
    ///  - 9 rows
    ///  - 9 cols
    let freq_map = HashMap::from([
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
    ]);

    let mut row_maps = vec![freq_map.clone(); 9];
    let mut col_maps = vec![freq_map.clone(); 9];
    let mut box_maps = vec![freq_map.clone(); 9];

    if let Ok(_) = init_state(input, &mut row_maps, &mut col_maps, &mut box_maps) {
        println!("Initialized state...")
    } else {
        panic!("Something went wrong with state init...")
    }

    vec![vec![]]
}

fn init_state(
    input: Vec<Vec<i32>>,
    row_maps: &mut Vec<HashMap<i32, i32>>,
    col_maps: &mut Vec<HashMap<i32, i32>>,
    box_maps: &mut Vec<HashMap<i32, i32>>,
) -> Result<()> {
    if let Ok(_) = init_row_maps(&input, row_maps) {
        println!("Initialized row maps.")
    } else {
        return Err(());
    }

    if let Ok(_) = init_col_maps(&input, col_maps) {
        println!("Initialized col maps.")
    } else {
        return Err(());
    }

    if let Ok(_) = init_box_maps(&input, box_maps) {
        println!("Initialized box maps.")
    } else {
        return Err(());
    }

    Ok(())
}

fn init_box_maps(input: &Vec<Vec<i32>>, box_maps: &mut Vec<HashMap<i32, i32>>) -> Result<()> {
    // 1, 1 (1 // 3 , 1 // 3) -> (0, 0) -> (1, 1) -> 1st row of boxes, 1st column of boxes
    // 1, 4 (1 // 3 , 4 // 3) -> (0, 1) -> (1, 2) -> 1st row of boxes, 2nd column of boxes
    // 7, 7 (7 // 3 , 7 // 3) -> (2, 2) -> (3, 3) -> 3rd row of boxes, 3rd column of boxes

    // [  0  ][  1  ][  2  ]
    // [  3  ][  4  ][  5  ]
    // [  6  ][  7  ][  8  ]

    for (r, row) in input.iter().enumerate() {
        for (c, el) in row.iter().enumerate() {
            let box_index = (r / 3) * 3 + (c / 3);
            if *el != 0 {
                box_maps[box_index].insert(*el, 1);
            }
        }
    }

    Ok(())
}

fn init_col_maps(input: &Vec<Vec<i32>>, col_maps: &mut Vec<HashMap<i32, i32>>) -> Result<()> {
    for (_, row) in input.iter().enumerate() {
        for (c, el) in row.iter().enumerate() {
            if *el != 0 {
                col_maps[c].insert(*el, 1);
            }
        }
    }

    Ok(())
}

fn init_row_maps(input: &Vec<Vec<i32>>, row_maps: &mut Vec<HashMap<i32, i32>>) -> Result<()> {
    for (i, r) in input.iter().enumerate() {
        for el in r {
            if *el != 0 {
                row_maps[i].insert(*el, 1);
            }
        }
    }

    Ok(())
}

fn solve(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![vec![]]
}

mod tests {
    use super::*;

    #[test]
    fn test() {
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

        assert_eq!(expected, solve(input))
    }
}
