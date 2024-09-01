use std::io::Stdout;
use crossterm::{style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool){
    // Force clear
    if force{
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    
}
