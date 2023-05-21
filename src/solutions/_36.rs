use std::collections::{HashMap, HashSet};

use super::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    continue;
                }

                if rows
                    .get(&row)
                    .map(|set| set.contains(&board[row][col]))
                    .unwrap_or(false)
                    || cols
                        .get(&col)
                        .map(|set| set.contains(&board[row][col]))
                        .unwrap_or(false)
                    || squares
                        .get(&(row / 3, col / 3))
                        .map(|set| set.contains(&board[row][col]))
                        .unwrap_or(false)
                {
                    return false;
                }

                cols.entry(col).or_default().insert(board[row][col]);
                rows.entry(row).or_default().insert(board[row][col]);
                squares
                    .entry((row / 3, col / 3))
                    .or_default()
                    .insert(board[row][col]);
            }
        }
        true
    }
}

#[test]
fn test_is_valid_sudoku() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert_eq!(Solution::is_valid_sudoku(board), true);

    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert_eq!(Solution::is_valid_sudoku(board), false);
}
