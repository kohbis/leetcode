use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<&char>> = vec![HashSet::with_capacity(9); 9];
        let mut cols: Vec<HashSet<&char>> = vec![HashSet::with_capacity(9); 9];
        let mut boxes: Vec<Vec<HashSet<&char>>> = vec![vec![HashSet::with_capacity(9); 3]; 3];

        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, x) in row.iter().enumerate() {
                if (x == &'.') {
                    continue;
                }

                if rows[row_idx].contains(x) {
                    return false;
                } else {
                    rows[row_idx].insert(x);
                }

                if cols[col_idx].contains(x) {
                    return false;
                } else {
                    cols[col_idx].insert(x);
                }

                if boxes[row_idx / 3][col_idx / 3].contains(x) {
                    return false;
                } else {
                    boxes[row_idx / 3][col_idx / 3].insert(x);
                }
            }
        }

        true
    }
}
