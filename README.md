Space Invaders Game

About:

This is a Space Invaders clone built with Rust and Dioxus. The game features classic Space Invaders gameplay with a modern twist, including:

Player-controlled ship that moves horizontally along the bottom of the screen
Aliens that move in formation and descend toward the player
Defensive shields that can be damaged by both player and alien projectiles
Mystery ship that occasionally appears for bonus points
Score tracking and high score persistence
Multiple levels with increasing difficulty

Features

Responsive controls using keyboard input

Animated sprites for all game elements

Destructible shields with letter-shaped patterns

Increasing game difficulty as levels progress

Game state management with proper separation of concerns

Comprehensive test suite with unit and integration tests


How to Play

Controls

Left/Right Arrow Keys: Move your ship horizontally

Shift Key: Fire projectiles

Enter Key: Start the game from the title screen

Tab Key: Restart after game over


Gameplay

Destroy all aliens to advance to the next level

Avoid alien projectiles

Don't let aliens reach the bottom of the screen

Try to achieve the highest score possible


Installation - 

Prerequisites:

Rust (latest stable version)

Install the Dioxus CLI with: cargo binstall dioxus-cli (recommended) or cargo install dioxus-cli


Building from Source: 

Clone the repository: git clone https://github.com/snikhil-vaddhi/space-invaders-game.git

cd space-invaders

Run the game in development mode: dx serve

Build the game for release:: dx build --release

Bundle the game for distribution: dx bundle --release

Development: 

Project Structure: 

src/game/: Game logic and state management

src/game/entities/: Game entities (player, aliens, projectiles, etc.)

src/game/logic.rs: Core game logic

src/rendering/: UI components and rendering

src/input/: Input handling

src/utils/: Utility functions and helpers


Running Tests

dx test

Hot Reloading - 
During development, you can use the Dioxus hot reloading feature: dx serve --hot

Credits: 
Game design inspired by the classic 1978 Space Invaders arcade game
Built with Dioxus - a Rust-based UI framework

License: 
This project is licensed under the MIT License - see the LICENSE file for details.
