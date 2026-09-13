use std::{io, time::{Duration, Instant}, vec};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use snake_game::{Direction, Game};

fn main() {
    println!("Snake Game");
    let mut stdout = io::stdout(); 

    // terminal setup
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let (cols, rows ) = terminal::size().unwrap();   
    
    let mut game = Game {
        snake: vec![(25, 25)],
        food: ( 10, 10),
        width: cols,
        height: rows,
        score: 0,
        is_paused: false,
        is_live: true
    }; 

    let tick_rate = Duration::from_millis(150);
    let last_tick = Instant::now(); 


    while game.is_live {
        let timeout= tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or(Duration::from_secs(0)); 
    
        if event::poll(timeout).unwrap() {
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
        game.draw(&mut stdout).unwrap(); 
    }


    terminal::disable_raw_mode().unwrap(); 
    // execute!(stdout, LeaveAlternateScreen).unwrap(); 
    println!("Game Over. Score: {}", game.score);

}
