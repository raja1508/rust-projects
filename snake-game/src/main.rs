use std::{io, time::{Duration}, vec};
use crossterm::{cursor, event::{self, Event, KeyCode}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use snake_game::{Direction, Game};

fn main() {
    println!("Snake Game");

    // Terminal Setup
    let mut stdout = io::stdout(); 
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    let (cols, rows ) = terminal::size().unwrap();   
    
    let mut game = Game {
        snake: vec![(cols/2 , 10)],
        food: ( 10, 10),
        width: cols,
        height: rows - 2,
        score: 0,
        is_paused: false,
        is_live: true,
        last_trail: None,
        wall: vec![(0,0)]
    }; 

    game.generate_food();
    
    // let tick_rate = Duration::from_millis(150);
    // let mut last_tick = Instant::now(); 
    
    game.draw_board(&mut stdout).unwrap(); 
    
    while game.is_live {
        // let timeout= tick_rate
        // .checked_sub(last_tick.elapsed())
        // .unwrap_or(Duration::from_secs(0)); 
    
    if event::poll(Duration::from_millis(150)).unwrap() {
                if let Event::Key(key_event)  = event::read().unwrap(){
                match key_event.code {
                    KeyCode::Char(' ') => game.resume_or_pause(),
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    KeyCode::Esc => {
                        game.is_live = false; 
                    }, 
                    _ => {}
                    
                }
            } 
        }

        
        // if last_tick.elapsed() > tick_rate {

        if !game.is_paused {
            game.draw_snake(&mut stdout).unwrap(); 
            game.draw_food(&mut stdout).unwrap();
        }
            // last_tick = Instant::now();  
        // }
    }

    terminal::disable_raw_mode().unwrap(); 
    execute!(stdout, cursor::Show).unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap(); 
    println!("Game Over. Score: {}", game.score);

}
