use std::{thread, time::Duration};
 
const WIDTH: usize = 20;
const HEIGHT: usize = 10;
 
type Grid = [[bool; WIDTH]; HEIGHT];
 
fn print_grid(grid: &Grid) {
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "⬛" } else { "⬜" });
        }
        println!();
    }
}
 
fn count_neighbors(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
 
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
 
            if nx < WIDTH && ny < HEIGHT && grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}
 
fn step(current: &Grid) -> Grid {
    let mut next = [[false; WIDTH]; HEIGHT];
 
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = current[y][x];
            let neighbors = count_neighbors(current, x, y);
 
            next[y][x] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
 
    next
}
 
fn main() {
    println!("🧬 Conway’s Game of Life - Press Ctrl+C to stop");
 
    let mut grid: Grid = [[false; WIDTH]; HEIGHT];
 
    // Seed: glider pattern
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;
 
    loop {
        print_grid(&grid);
        grid = step(&grid);
        thread::sleep(Duration::from_millis(300));
    }
}
