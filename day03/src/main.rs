use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

/// Counts number of trees encountered going down slope.
fn count_trees(grid: &Vec<Vec<char>>, dx: usize, dy: usize) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut curr_row = 0;
    let mut curr_col = 0;
    let mut n_tree = 0;

    while curr_row < height && curr_col < width {
        if grid[curr_row][curr_col] == '#' {
            n_tree += 1
        }
        curr_row += dy;
        curr_col = (curr_col + dx) % width;
    }

    n_tree
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    loop {
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let mut row: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            row.push(c);
        }
        grid.push(row);
        line.clear();
    }

    println!("{}", count_trees(&grid, 3, 1));
    println!(
        "{}",
        count_trees(&grid, 1, 1) as i64
            * count_trees(&grid, 3, 1) as i64
            * count_trees(&grid, 5, 1) as i64
            * count_trees(&grid, 7, 1) as i64
            * count_trees(&grid, 1, 2) as i64
    );

    Ok(())
}
