pub mod Print {
    use crate::Cell;
    use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}};
    use std::io::{Write, stdout};

    pub fn print_grid(grid: [[Cell; 31]; 31], point: i32) {
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
        println!("\rHigh Score: {point}");
        let _ = stdout.execute(Clear(ClearType::Purge));
        let _ = stdout.flush();
    }
}
