mod keyboard_controll;
pub use crate::keyboard_controll::Keyboard::keyboard;

use std::{io::{Write, stdout}, os::windows::thread, thread::{sleep, spawn}, time::Duration};
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
    let mut x: usize = rand::rng().random_range(0..30);
    //let mut y: usize = rand:: rng().random_range(0..30);
    // let mut pos_x = 0;
    // let mut pos_y = 0;
    
    let mut i: usize = 0;

    // let pos_x = spawn(|| {
    //     let input = keyboard();

    //     match input {
    //         "UP" => {pos_x = 0; pos_y = -1},
    //         "DOWN" => {pos_x = 0; pos_y = 1},
    //         "LEFT" => {pos_x = -1; pos_y = 0},
    //         "DOWN" => {pos_x = 1; pos_y = 0},
    //     }
    // });

    loop {
        for s in 0..snake.len() {
            grid[i+s][3] = Cell::Snake;
        }

        if grid[x][3] == Cell::Snake {
            snake.push("o");
            x = rand::rng().random_range(0..30);
            //y = rand::rng().random_range(0..30);
        }

        grid[x][3] = Cell::Fruit;

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