use std::io::{self, BufRead};
fn main() {
    let grid = vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];
    let stdin = io::stdin();
    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    // println!("{:?}", grid);
    println!("Occurrences: {}", part_one(grid.clone()));
    println!("Occurrences: {}", part_two(grid));
}

fn part_two(grid: Vec<Vec<char>>) -> i32 {
    // Iterate over each cell in the grid
    let n = grid.len();
    let m = if n > 0 { grid[0].len() } else { 0 };
    let mut count = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if is_x_mas(&grid, i, j) {
                count += 1;
            }
        }
    }
    return count;
}
fn is_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // Check if the center is 'A' and diagonals match the X-MAS pattern
    grid[x][y] == 'A'
        && ((grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
            || (grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M'))
        && ((grid[x + 1][y - 1] == 'M' && grid[x - 1][y + 1] == 'S')
            || (grid[x + 1][y - 1] == 'S' && grid[x - 1][y + 1] == 'M'))
}
fn part_one(grid: Vec<Vec<char>>) -> i32 {
    let target = "XMAS";
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
    ];

    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            for &(dx, dy) in &directions {
                if matches_pattern(&grid, target, i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn matches_pattern(
    grid: &Vec<Vec<char>>,
    target: &str,
    start_x: usize,
    start_y: usize,
    dx: i32,
    dy: i32,
) -> bool {
    let target: Vec<char> = target.chars().collect();
    let n = grid.len();
    let m = grid[0].len();

    for (k, &ch) in target.iter().enumerate() {
        let x = start_x as i32 + k as i32 * dx;
        let y = start_y as i32 + k as i32 * dy;

        // Check bounds
        if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
            return false;
        }

        if grid[x as usize][y as usize] != ch {
            return false;
        }
    }

    true
}
