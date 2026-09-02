# Sudoku Solver

A Rust implementation of a Sudoku solver: given a partially filled 9x9 board, fill in the empty cells so the board satisfies all Sudoku rules.

## Rules

A valid solution must satisfy:

- Each of the digits `1-9` occurs exactly once in each row.
- Each of the digits `1-9` occurs exactly once in each column.
- Each of the digits `1-9` occurs exactly once in each of the nine 3x3 sub-boxes of the grid.
- The `.` character indicates an empty cell.

## Example

Input:

```
[["5","3",".",".","7",".",".",".","."],
 ["6",".",".","1","9","5",".",".","."],
 [".","9","8",".",".",".",".","6","."],
 ["8",".",".",".","6",".",".",".","3"],
 ["4",".",".","8",".","3",".",".","1"],
 ["7",".",".",".","2",".",".",".","6"],
 [".","6",".",".",".",".","2","8","."],
 [".",".",".","4","1","9",".",".","5"],
 [".",".",".",".","8",".",".","7","9"]]
```

Output:

```
[["5","3","4","6","7","8","9","1","2"],
 ["6","7","2","1","9","5","3","4","8"],
 ["1","9","8","3","4","2","5","6","7"],
 ["8","5","9","7","6","1","4","2","3"],
 ["4","2","6","8","5","3","7","9","1"],
 ["7","1","3","9","2","4","8","5","6"],
 ["9","6","1","5","3","7","2","8","4"],
 ["2","8","7","4","1","9","6","3","5"],
 ["3","4","5","2","8","6","1","7","9"]]
```

## Constraints

- `board.length == 9`
- `board[i].length == 9`
- `board[i][j]` is a digit or `.`
- The input board is guaranteed to have exactly one solution.

## Approach

The solver (`solver.rs`) uses backtracking with bitmask constraint tracking:

- Each row, column, and 3x3 box keeps a `u16` bitmask of digits already placed.
- Empty cells are solved in order of fewest remaining candidates first (a minimum-remaining-values heuristic), which prunes the search space significantly compared to solving cells in a fixed order.
