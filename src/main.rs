mod keyboard_controll;
pub use crate::keyboard_controll::Keyboard::keyboard;

use std::{io::{stdout, Write}, thread::sleep, time::Duration,};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Snake,
    Fruit,
}

fn main() {
    let mut grid: [[Cell; 31]; 31] = [[Cell::Empty; 31]; 31];
    let mut snake:  Vec<&str> = vec!["O", "o", "o"];
    let mut stdout = stdout();
    let mut x: usize = rand::rng().random_range(0..30);
    let mut y: usize = rand:: rng().random_range(0..30);
    
    let mut i: usize = 0;

    loop {
        for s in 0..snake.len() {
            grid[i+s][3] = Cell::Snake;
        }

        grid[x][y] = Cell::Fruit;

        if grid[x][y] == Cell::Snake {
            snake.push("o");
            x = rand::rng().random_range(0..30);
            y = rand::rng().random_range(0..30);
        }

        print_grid(grid);

        if grid[30][3] == Cell::Snake {
            break;
        }

        grid[i][3] = Cell::Empty;
        i += 1;

        sleep(Duration::from_millis(500));
    }
}

fn print_grid(grid: [[Cell; 31]; 31]) {
    let mut stdout = stdout();
    for row in grid.iter() {
        for &cell in row.iter() {
            let symbol = match cell {
                Cell::Empty => ".",
                Cell::Snake => "O",
                Cell::Fruit => "*",
            };
            print!("{}", symbol);
        }
        println!();
    }
    stdout.flush().unwrap();
}