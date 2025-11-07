pub mod Collision {
    use crate::Cell;

    pub enum CollisionType {
        None,
        Wall,
        SelfCollision,
        FruitCollision,
    }

    pub fn collision_check(
        snake: Vec<(i32, i32)>,
        new_head: (i32, i32),
        px: i32,
        grid: [[Cell; 31]; 31],
        x: usize,
        y: usize,
    ) -> CollisionType {
        if snake[0].0 < 0 || snake[0].1 < 0 || snake[0].0 >= 31 || snake[0].1 >= 31 {
            return CollisionType::Wall;
        }

        let check_x = new_head.0 + px + 1;
        let check_y = new_head.1;

        // Verifica se está dentro do grid antes de acessar
        if check_x >= 0 && check_x < 31 && check_y >= 0 && check_y < 31 {
            if grid[check_x as usize][check_y as usize] == Cell::Snake {
                return CollisionType::SelfCollision;
            }
        }

        if grid[x][y] == Cell::Snake {
            return CollisionType::FruitCollision;
        }

        CollisionType::None
    }
}
