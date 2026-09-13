use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use crate::Game;


impl Game {
pub fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
    // no Clear::All here anymore — do it once at startup instead

    // border only needs to be drawn once too — move it out of the
    // per-frame draw and draw it a single time before the loop starts

    // --- erase previous frame's dynamic content ---
    // (requires storing previous snake body + previous food if it moved)
    // simplest: just blank the interior region each frame instead of
    // the whole terminal, since interior is much smaller than a full clear
    for y in 1..=self.height {
        queue!(stdout, MoveTo(1, y), Print(" ".repeat(self.width as usize)))?;
    }

    // --- draw snake ---
    queue!(stdout, SetForegroundColor(Color::Green))?;
    for &(x, y) in &self.snake {
        queue!(stdout, MoveTo(x + 1, y + 1), Print("■"))?;
    }
    queue!(stdout, ResetColor)?;

    // --- draw food ---
    queue!(stdout, SetForegroundColor(Color::Red))?;
    queue!(stdout, MoveTo(self.food.0 + 1, self.food.1 + 1), Print("●"))?;
    queue!(stdout, ResetColor)?;

    // --- score / status ---
    queue!(stdout, MoveTo(0, self.height + 3), Print(format!("Score: {}   ", self.score)))?;

    if self.is_paused {
        queue!(stdout, MoveTo(0, self.height + 4), Print("-- PAUSED --   "))?;
    } else if !self.is_live {
        queue!(stdout, MoveTo(0, self.height + 4), Print("-- GAME OVER --"))?;
    } else {
        queue!(stdout, MoveTo(0, self.height + 4), Print("               "))?;
    }

    stdout.flush()?;
    Ok(())
}
}