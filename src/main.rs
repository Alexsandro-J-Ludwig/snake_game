mod collision;
mod keyboard;
mod print;
mod render;

use crate::{
    collision::Collision::{CollisionType, collision_check},
    keyboard::{Keyboard::Moves, Keyboard::keyboard},
    print::Print::print_grid,
    render::Render::render,
};

use rand::Rng;
use std::{thread::sleep, time::Duration};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Snake,
    Fruit,
}

fn main() {
    let mut grid: [[Cell; 31]; 31] = [[Cell::Empty; 31]; 31];
    let mut snake: Vec<(i32, i32)> = vec![(14, 14), (14, 15), (14, 16)];
    let mut x: usize = rand::rng().random_range(0..30);
    let mut y: usize = rand::rng().random_range(0..27);

    let mut px = 0;
    let mut py = 0;

    let mut points = 0;

    loop {
        grid = render(grid, snake.clone());

        //Catch the keyboard arrows keycaps
        let position: Moves = keyboard();

        match keyboard() {
            Moves::Up => (px, py) = (-1, 0),
            Moves::Down => (px, py) = (1, 0),
            Moves::Left => (px, py) = (0, -1),
            Moves::Right => (px, py) = (0, 1),
            Moves::None => {}
        }

        //Add a new head in snake body
        let new_head = (snake[0].0 + px, snake[0].1 + py);

        //Verify if snake colisions in game
        match collision_check(snake.clone(), new_head, px, grid, x, y) {
            CollisionType::Wall => {
                println!("💥 Você bateu na parede - Game Over!");
                println!("🏆 Pontuação final: {points}");
                break;
            }
            CollisionType::SelfCollision => {
                println!("🐍 Você se mordeu - Game Over!");
                println!("🏆 Pontuação final: {points}");
                break;
            }
            CollisionType::FruitCollision => {
                snake.push(*snake.last().unwrap());
                x = rand::rng().random_range(0..30);
                y = rand::rng().random_range(0..30);
                points += 1;
            }
            CollisionType::None => {
                snake.insert(0, new_head);
                snake.pop();
            }
        }

        grid[x][y] = Cell::Fruit;
        print_grid(grid, points);
        sleep(Duration::from_millis(20));
    }
}
