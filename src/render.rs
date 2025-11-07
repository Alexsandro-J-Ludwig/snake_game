pub mod Render {
    use std::{thread::sleep, time::Duration};

    use crate::Cell;

    pub fn render(mut grid: [[Cell; 31];31], snake: Vec<(i32, i32)>) -> [[Cell; 31];31]{
        for row in &mut grid {
            for cell in row.iter_mut() {
                *cell = Cell::Empty;
            }
        }

        //snake render
        for &(x, y) in &snake {
            if x >= 0 && y >= 0 && x < 31 && y < 31 {
                grid[x as usize][y as usize] = Cell::Snake;
            }
        }
        sleep(Duration::from_millis(100));

        grid
    }
}