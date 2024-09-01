use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::{event, terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use space_invaders::frame::new_frame;
use space_invaders::{frame, render};

enum State {
    Alive,
    Dead,
}

struct Position{
    x: i32,
    y: i32,
}

enum GameObject{
    Player(Position, State),
    Enemy(Position, State),
    Bullet(Position, State),
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a seperate thread 
    // for practice of multi-threading
    let(render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame limit
        let curr_frame = new_frame();
        
        // Input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code {
                    KeyCode::Esc => {
                         break 'gameloop;
                    },
                    _ => continue,
                }
            }
        }

        // Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
