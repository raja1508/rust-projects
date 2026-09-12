use crossterm::event::{self, Event, KeyCode};
use snake_game::{Direction, Game};
use rand::Rng; 
fn main() {
    println!("Snake Game");
    
    let game = Game {
        snake: Vec::new(),
        direction: Direction::Down,
        food: ( x , y),
        width: 0,
        height: 0,
        score: 0
    }; 
    let x = rand::thread_rng().gen_range(1..=100) as u16;
    let y = rand::thread_rng().gen_range(1..=100) as u16; 
    game.set_food_at(x, y); 


    // if let Event::Key(key_event)  = event::read().unwrap(){
    //     match key_event.code {
    //         KeyCode::Up => game.change_direction(Direction::Up),
    //         KeyCode::Down => game.change_direction(Direction::Down),
    //         KeyCode::Left => game.change_direction(Direction::Left),
    //         KeyCode::Right => game.change_direction(Direction::Right),
    //         KeyCode::Esc => break, 
    //         _ => {}

    //     }
    // } 
}
