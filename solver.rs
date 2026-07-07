impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        
        let mut empty_cells = Vec::with_capacity(81);

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];
                if val != '.' {
                    let digit = val.to_digit(10).unwrap() as usize;
                    let mask = 1 << digit;
                    rows[r] |= mask;
                    cols[c] |= mask;
                    boxes[(r / 3) * 3 + (c / 3)] |= mask;
                } else {
                    empty_cells.push((r, c));
                }
            }
        }

        Self::solve(board, &mut rows, &mut cols, &mut boxes, &mut empty_cells);
    }

    fn solve(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
        empty_cells: &mut Vec<(usize, usize)>
    ) -> bool {
        if empty_cells.is_empty() {
            return true;
        }

        let mut best_idx = 0;
        let mut best_candidates = 0x3FE;
        let mut min_choices = 10;

        for (idx, &(r, c)) in empty_cells.iter().enumerate() {
            let b = (r / 3) * 3 + (c / 3);
            let used = rows[r] | cols[c] | boxes[b];
            let candidates = (!used) & 0x3FE;
            
            let choices = candidates.count_ones(); 

            if choices < min_choices {
                min_choices = choices;
                best_candidates = candidates;
                best_idx = idx;
            }
            if min_choices == 0 {
                return false;
            }
        }

        let (r, c) = empty_cells.remove(best_idx);
        let b = (r / 3) * 3 + (c / 3);

        for digit in 1..=9 {
            if (best_candidates & (1 << digit)) != 0 {
                let mask = 1 << digit;

                board[r][c] = std::char::from_digit(digit as u32, 10).unwrap();
                rows[r] |= mask;
                cols[c] |= mask;
                boxes[b] |= mask;

                if Self::solve(board, rows, cols, boxes, empty_cells) {
                    return true;
                }

                rows[r] &= !mask;
                cols[c] &= !mask;
                boxes[b] &= !mask;
            }
        }

        board[r][c] = '.';
        empty_cells.insert(best_idx, (r, c));
        false
    }
}