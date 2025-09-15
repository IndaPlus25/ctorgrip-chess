use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub enum Color {
    White,
    Black,
}

pub struct Game {
    state: GameState,
    black: u64,
    white: u64,
    kings: u64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
        }
    }

    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60);

        None
    }

    pub fn set_promotion(&mut self, piece: String) {}

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    pub fn get_possible_moves(&self, postion: String) -> Option<Vec<String>> {
        None
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    #[test]
    fn game_progress_sanity() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
