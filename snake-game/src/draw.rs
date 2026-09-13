use crossterm::{
    cursor::{self}, execute, queue, style::{self, Stylize}, terminal,
};
use std::io::{self, Write};
use crate::Game;


impl Game {
    pub fn draw_board(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        self.wall.pop(); 
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?; 
        // execute!(stdout, SetForegroundColor(Color::Blue))?; 
        for x in 0..=self.width {
            for y in 0..=self.height {
                if x == 0 || x == self.width || y == 0 || y == self.height {
                    self.wall.push((x,y));
                    queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("█".magenta()))?; 
                }
            }
        }
        stdout.flush()?; 
        Ok(())
    }

    pub fn draw_food(&self , stdout: &mut io::Stdout) -> io::Result<()>{
        let (x,y) = self.food; 
        queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("▣".red()))?;
        stdout.flush()?; 
        Ok(())
    }

    pub fn draw_snake(&self , stdout: &mut io::Stdout) -> io::Result<()>{
        if let Some((x, y)) = self.last_trail {
            queue!(stdout, cursor::MoveTo(x, y), style::Print(" "))?; 
        }
        for value in self.snake.iter(){
            let (x, y ) = value; 
            queue!(stdout, cursor::MoveTo(*x, *y), style::PrintStyledContent("■".green()))?;
        }
        stdout.flush()?; 
        Ok(())
    }
}