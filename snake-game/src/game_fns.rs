use std::{collections::VecDeque};

use rand::Rng;

use crate::{Direction, Game};


impl Game {
    pub fn change_direction(&mut self, direction: Direction) {
        let (x, y) = self.snake.get(0).unwrap(); 
        let (nx, ny) = match direction {
            Direction::Up => ( *x, y - 1),
            Direction::Down => (*x, y + 1),
            Direction::Left => ( x - 1, *y),
            Direction::Right => (x + 1, *y)
        }; 

        let new_head = (nx, ny); 
        if self.snake.contains(&new_head) {
            self.is_live = false; 
            return
        }

        if self.wall.contains(&new_head){
            self.is_live = false; 
            return;
        }

        if self.food == new_head {
            // grow the snake and 
            // generate new food location
            self.generate_food(); 
            self.score += 1; 
 
        }else {
            let x = self.snake.pop().unwrap();
            self.last_trail = Some(x); 
        };

        let mut snake = VecDeque::from(self.snake.clone()); 
        snake.push_front(new_head);
        self.snake = Vec::from(snake);
    
    }

    pub fn resume_or_pause(&mut self) {
        let x = if self.is_paused { false } else { true }; 
        self.is_paused = x ; 
    }

    pub fn generate_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(0..self.width - 2); 
            let y = rand::thread_rng().gen_range(0..self.height - 2);

            if !self.snake.contains(&(x, y)) && !self.wall.contains(&(x, y)){
                self.food = (x, y);  
                break;
            }
            
        }
    }
}