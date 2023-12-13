use crate::snake::{SnakeAI, SnakeGame, SnakeMove};
use rand::Rng;

pub struct DumbAI;

impl SnakeAI for DumbAI {
    fn next_move(&self, game: &SnakeGame) -> SnakeMove {
        if game.snake.head.0 < game.apple.0 {
            SnakeMove::Right
        } else if game.snake.head.0 > game.apple.0 {
            SnakeMove::Left
        } else if game.snake.head.1 < game.apple.1 {
            SnakeMove::Down
        } else {
            SnakeMove::Up
        }
    }
}

pub struct RandomAI;

impl SnakeAI for RandomAI {
    fn next_move(&self, game: &SnakeGame) -> SnakeMove {
        match game.rng.clone().gen_range(0..4) {
            0 => SnakeMove::Up,
            1 => SnakeMove::Down,
            2 => SnakeMove::Left,
            3 => SnakeMove::Right,
            _ => unreachable!(),
        }
    }
}
