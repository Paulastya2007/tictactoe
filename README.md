# 🕹️ Tic-Tac-Toe: Rust Edition

A premium, high-polish Tic-Tac-Toe game built with the **Rust** programming language and the **Macroquad** game engine. This isn't just a basic grid—it's a "juicy" experience featuring dynamic animations, custom particle effects, multiple visual themes, and an unbeatable AI.

---

## 🌟 For the Players (Non-Technical Overview)

### How the Game Works
The goal is simple: be the first to get three of your symbols (X or O) in a row, column, or diagonal. 
- **Play vs Friend**: Local multiplayer mode for two people sharing the same screen.
- **Play vs AI**: Challenge the computer! You can even choose whether you want to be X or O.
- **Visual Themes**: Bored of the same look? Press **'T'** to cycle through 4 beautiful themes (Modern, Neon Night, Royal Gold, and Terminal).

### The "Juice" (What makes it feel good)
- **Animations**: Pieces don't just appear; they "pop" onto the board with a bouncy effect.
- **Winning Moment**: When someone wins, the winning line is highlighted with a glowing strike, and a celebratory explosion of particles fills the screen.
- **Atmosphere**: Moving background glows and smooth transitions make the game feel alive.

### The "Unbeatable" Brain (Minimax)
Ever wonder how the computer always seems to win or force a draw? It uses an algorithm called **Minimax**. 
Imagine the AI looking into the future. It plays out every possible move until the end of the game in its "mind." It calculates:
1. If I move here, can the human win? (Avoid this!)
2. If I move here, can I force a win? (Do this!)
Because Tic-Tac-Toe is a mathematically "solved" game, the AI knows every single outcome before it even makes its first move. You can't beat it—at best, you can only hope for a draw!

---

## 🛠️ For the Developers (Technical Deep Dive)

### Architecture
- **State Management**: The game uses a finite state machine (`src/state.rs`) to transition between the Menu, Symbol Selection, and Gameplay phases.
- **Global State**: We utilize `static mut` and `OnceLock` patterns for performance and ease of access to shared resources like textures, fonts, and the game board. 
  - *Note: This involves `unsafe` Rust blocks for global state access, keeping the procedural logic simple and efficient.*
- **Custom Particles**: Instead of using heavy libraries, we implemented a custom particle physics engine (`src/particles.rs`) with drag, scaling, and lifetime management.
- **Theme System**: A centralized `theme.rs` module maps colors to UI elements, allowing for instant, zero-latency visual swaps.

### Features for Contributors
- **Virtual Resolution**: The game renders to a fixed 800x600 virtual canvas and auto-scales/letterboxes to fit any physical window size (`src/utils.rs`).
- **Embedded Assets**: All fonts, textures, and sounds are embedded into the compiled binary using `include_bytes!`, meaning the finished game is a single, portable `.exe`.

### How to Run
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Run the following command in the terminal:
   ```bash
   cargo run
   ```


---

*Made with 🦀 and ❤️*
