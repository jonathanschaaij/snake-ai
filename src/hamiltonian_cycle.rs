use crate::snake::{SnakeAI, SnakeGame, SnakeMove};
use rand::Rng;

pub struct HamiltonianCycleAI;

impl SnakeAI for HamiltonianCycleAI {
    fn next_move(&self, game: &SnakeGame) -> SnakeMove {
        //Hardcoded hamiltonian cycle for 10x10 grid

        // Move to first column via top
        if game.snake.head.1 == 0 {
            if game.snake.head.0 == 0 {
                return SnakeMove::Down;
            } else {
                return SnakeMove::Left;
            }
        } else if game.snake.head.1 == 9 {
            if game.snake.head.0 % 2 == 0 {
                return SnakeMove::Right;
            } else {
                return SnakeMove::Up;
            }
        } else if game.snake.head.1 == 1 {
            if game.snake.head.0 % 2 == 0 {
                return SnakeMove::Down;
            } else {
                if game.snake.head.0 == 9 {
                    return SnakeMove::Up;
                } else {
                    return SnakeMove::Right;
                }
            }
        } else {
            if game.snake.head.0 % 2 == 0 {
                return SnakeMove::Down;
            } else {
                return SnakeMove::Up;
            }
        }
    }
}
