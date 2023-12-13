use core::fmt;
use rand::Rng;

const SIZE: usize = 10;

pub struct Snake {
    pub head: (i32, i32),
    pub body: Vec<(i32, i32)>,
}

impl Snake {
    pub fn len(&self) -> usize {
        self.body.len() + 1
    }
}

#[derive(Debug)]
pub enum SnakeMove {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    pub snake: Snake,
    pub apple: (i32, i32),
    pub rng: rand::rngs::ThreadRng,
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        let mut rng = rand::thread_rng();
        let mut snake = Snake {
            head: (rng.gen_range(0..SIZE as i32), rng.gen_range(0..SIZE as i32)),
            body: Vec::new(),
        };
        let mut apple = (rng.gen_range(0..SIZE as i32), rng.gen_range(0..SIZE as i32));
        SnakeGame { snake, apple, rng }
    }

    pub fn update(&mut self, snake_move: SnakeMove) -> GameEvent {
        // Update snake position and return true if game over
        self.snake.body.push(self.snake.head);
        match snake_move {
            SnakeMove::Up => {
                self.snake.head.1 -= 1;
            }
            SnakeMove::Down => {
                self.snake.head.1 += 1;
            }
            SnakeMove::Left => {
                self.snake.head.0 -= 1;
            }
            SnakeMove::Right => {
                self.snake.head.0 += 1;
            }
        }
        if self.snake.head == self.apple {
            self.apple = (
                self.rng.gen_range(0..SIZE as i32),
                self.rng.gen_range(0..SIZE as i32),
            );
            return GameEvent::Eating;
        }
        self.snake.body.remove(0);
        if self.snake.head.0 < 0
            || self.snake.head.0 >= SIZE as i32
            || self.snake.head.1 < 0
            || self.snake.head.1 >= SIZE as i32
        {
            return GameEvent::Wall; // Game over
        }

        if self.snake.body.contains(&self.snake.head) {
            return GameEvent::Suicide; // Game over
        }
        return GameEvent::Moved;
    }
}

impl fmt::Display for SnakeGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = [["  "; 10]; 10];

        // Place the snake on the grid
        grid[self.snake.head.1 as usize][self.snake.head.0 as usize] = "😈";
        for &(x, y) in &self.snake.body {
            grid[y as usize][x as usize] = "🔴";
        }

        // Place the apple on the grid
        grid[self.apple.1 as usize][self.apple.0 as usize] = "🍏";

        // Display the grid
        for row in grid.iter() {
            for &cell in row.iter() {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }

        // Display snake length
        write!(f, "\nSnake Length: {}", self.snake.len())
    }
}

#[derive(Debug)]
pub enum GameEvent {
    Moved,
    Suicide,
    Wall,
    Eating,
}

pub trait SnakeAI {
    fn next_move(&self, game: &SnakeGame) -> SnakeMove;
}
