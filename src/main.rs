mod keyboard;
use crate::keyboard::{Keyboard::keyboard, Keyboard::Moves};

use std::{io::{Write, stdout}, thread::sleep, time::Duration};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Snake,
    Fruit,
}

fn main() {
    let mut grid: [[Cell; 31]; 31] = [[Cell::Empty; 31]; 31];
    let mut snake:  Vec<(i32, i32)> = vec![(14, 14), (14, 15), (14, 16)];
    let mut x: usize = rand::rng().random_range(0..30);
    let mut y: usize = rand:: rng().random_range(0..30);

    let mut pos_x: isize = 14;
    let mut pos_y: isize = 14;
    let mut px = 0;
    let mut py = 0;

    let mut point = 0;

    loop {
        for row in &mut grid {
            for cell in row.iter_mut() {
                *cell = Cell::Empty;
            }
        }
        for &(x, y) in &snake {
            if x >= 0 && y >= 0 && x < 31 && y < 31 {
                grid[x as usize][y as usize] = Cell::Snake;
            }
        }

        let position:Moves = keyboard();

        match position {
            Moves::Up => {px = -1; py = 0},
            Moves::Down => {px = 1; py = 0},
            Moves::Left => {px = 0; py = -1},
            Moves::Right => {px = 0; py = 1},
            Moves::None => {}
        }

        let new_head = (snake[0].0 + px, snake[0].1 + py);
        snake.insert(0, new_head);

        snake.pop();

        if grid[x][y] == Cell::Snake {
            snake.push(*snake.last().unwrap());
            x = rand::rng().random_range(0..30);
            y = rand::rng().random_range(0..30);
            point += 1;
        }

        grid[x][y] = Cell::Fruit;

        print_grid(grid, point);

        if pos_x < 0 || pos_y < 0 || pos_x >= 31 || pos_y >= 31 {
            println!("Game Over");
            break;
        }

        if grid[(snake[0].0 + 1) as usize][(snake[0].1) as usize] == Cell::Snake {
            println!("Game Over");
            break;
        }

        pos_x += px as isize;
        pos_y += py as isize;

        sleep(Duration::from_millis(200));
    }
}

fn print_grid(grid: [[Cell; 31]; 31], point: i32) {
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
    print!("\r{point}");
    stdout.flush().unwrap();
}