use rand::Rng;
use crate::{Direction, Game};

impl Game {
    pub fn change_direction(&mut self, direction: Direction) {
        let is_reverse = matches!(
            (self.moving_direction, direction),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        );

        if !is_reverse{
            self.changing_direction = Some(direction)
        }
    
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

    pub fn continue_movement(&mut self) {
        if let Some(direction) = self.changing_direction {
            let is_same_dir = matches!(
            (direction, self.moving_direction),
            (Direction::Up, Direction::Up)
                | (Direction::Down, Direction::Down)
                | (Direction::Left, Direction::Left)
                | (Direction::Right, Direction::Right)
            );

            if is_same_dir{
                self.changing_direction = None;
                return; 
            }else {
                self.moving_direction = direction; 
            }

        
        }
        
        let direction = self.moving_direction; 
        let new_head: (u16, u16) = self.new_head(&direction); 
 
        self.move_snake(new_head)
        
        
    }

    fn new_head(&mut self, direction: &Direction) -> (u16, u16) {
        let (x, y) = self.snake.get(0).unwrap(); 
        let (nx, ny) = match direction {
            Direction::Up => ( *x, y - 1),
            Direction::Down => (*x, y + 1),
            Direction::Left => ( x - 1, *y),
            Direction::Right => (x + 1, *y)
        }; 

        let new_head = (nx, ny);
        new_head 
    }

    fn move_snake(&mut self, new_head: (u16, u16)){
        if self.snake.contains(&new_head) {
            self.is_live = false; 
            return
        }

        if self.wall.contains(&new_head){
            self.is_live = false; 
            return;
        }

        if self.food == new_head {
            self.generate_food(); 
            self.score += 1; 
 
        }else {
            let x = self.snake.pop_back();
            if let Some(x) = x {
                self.last_trail = Some(x); 
            }else {
                self.last_trail = Some(new_head)
            }
        };

        self.snake.push_front(new_head);
    }
}