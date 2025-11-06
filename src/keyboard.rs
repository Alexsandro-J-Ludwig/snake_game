pub mod Keyboard {
    use crossterm::event::{self, Event, KeyCode};
    use std::time::Duration;
    pub enum Moves {
        Up,
        Down,
        Left,
        Right,
        None,
    }

    pub fn keyboard() -> Moves {
        // Verifica se há uma tecla pressionada (sem travar)
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => return Moves::Up,
                    KeyCode::Down => return Moves::Down,
                    KeyCode::Left => return Moves::Left,
                    KeyCode::Right => return Moves::Right,
                    _ => return Moves::None,
                }
            }
        }
    
        Moves::None
    }
}
