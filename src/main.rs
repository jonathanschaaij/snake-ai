mod dumb_ai;
mod hamiltonian_cycle;
mod snake;
use crate::snake::SnakeAI;

fn main() {
    println!("Starting a new game of snake!");

    // let snake_ai = dumb_ai::RandomAI;
    let snake_ai = hamiltonian_cycle::HamiltonianCycleAI;

    let n_cycles = 100;

    let mut total_scores = Vec::new();
    let mut average_steps_between_apples = Vec::new();

    for i in 0..n_cycles {
        println!("Starting game {}", i);
        let mut steps = 0;
        let mut game = snake::SnakeGame::new();
        loop {
            // println! {"{}", game};
            // println!("Snake length: {}", game.snake.len());
            // println!("Snake head: {:?}", game.snake.head);
            // println!("Apple: {:?}", game.apple);

            let snake_move = snake_ai.next_move(&game);
            // println!("Snake move: {:?}", snake_move);

            let game_event = game.update(snake_move);
            // println!("Game event: {:?}", game_event);

            match game_event {
                snake::GameEvent::Moved => {}
                snake::GameEvent::Eating => {
                    println!("Eating apple!")
                }
                _ => {
                    // println!("Game over!");
                    break;
                }
            }
            steps += 1;
        }
        total_scores.push(game.snake.len() as f32);
        average_steps_between_apples.push(steps as f32 / game.snake.len() as f32);
    }

    println!(
        "Average score: {}",
        total_scores.iter().sum::<f32>() / n_cycles as f32
    );
    println!(
        "Average steps between apples: {}",
        average_steps_between_apples.iter().sum::<f32>() / n_cycles as f32
    );
}
