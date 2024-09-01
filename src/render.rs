use std::io::{Stdout, Write};
use crossterm::{cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool){
    // Force clear
    if force{
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    // Enumerate and Compare
    for(x, curr_col) in curr_frame.iter().enumerate() {
        for (y, curr_row) in curr_col.iter().enumerate(){
            if *curr_row != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *curr_row)
            }
        }
    }

    stdout.flush().unwrap();
}
