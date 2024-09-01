# Space Invaders in Rust

This project is a simple implementation of the classic Space Invaders game using the Rust programming language. The game is played in the terminal and utilizes multi-threading for rendering and game logic. It also features sound effects using the `rusty_audio` crate.

## Features

- **Multi-threaded Rendering**: The rendering of frames is handled in a separate thread to allow smooth gameplay and responsive controls.
- **Sound Effects**: Includes sound effects for different game events such as shooting, explosions, and game over, enhancing the gaming experience.
- **Terminal Graphics**: Uses the Crossterm library to handle terminal graphics and input/output efficiently.

## How to Play

1. **Move Left**: Use the left arrow key.
2. **Move Right**: Use the right arrow key.
3. **Shoot**: Press the space bar or enter key.
4. **Exit**: Press the escape key.

The goal is to shoot down all the invading aliens before they reach the bottom of the screen. If they reach the bottom, you lose the game.

## Installation

To run this project, you need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions at [Rust's official website](https://www.rust-lang.org/).

Clone this repository and run the following command to build the project:

```bash
cargo build --release
```

To run the game:

```bash
cargo run --release
```

## Project Structure

- `main.rs`: The entry point of the game, handles the game loop and user input.
- `frame.rs`: Contains the logic for the game frame and rendering.
- `player.rs`: Implements the player character and controls.
- `invaders.rs`: Implements the enemy invaders' logic.
- `shot.rs`: Implements the projectiles fired by the player.
- `render.rs`: Manages rendering the game to the terminal.
- `sounds/`: Directory containing the sound files used in the game.

## Lessons Learned

This project was developed following the **Rust Programming Fundamentals** by Nathan Stocks course on Udemy as a guideline. However, I also made several modifications and added my own code to enhance my understanding and avoid directly copying the tutorial.

### Key Learnings:

- **Rust Basics**: Gained hands-on experience with Rust syntax and its unique features such as ownership, borrowing, and lifetimes.
- **Multi-threading in Rust**: Learned how to create and manage threads safely using Rust's concurrency model.
- **Error Handling**: Implemented robust error handling using `Result` and `Option` types.
- **Using Crates**: Gained experience using external crates such as `crossterm` for terminal control and `rusty_audio` for audio playback.
- **Game Development Fundamentals**: Developed a better understanding of basic game development concepts like the game loop, collision detection, and rendering.

This project was both a challenging and rewarding experience that helped solidify my understanding of Rust and basic game development principles.
