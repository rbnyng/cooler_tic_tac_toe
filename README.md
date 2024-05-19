# Cooler Tic Tac Toe

This project is a Tic Tac Toe game variant implemented in Rust. The game allows two players to take turns placing 'X' and 'O' on a 3x3 board with the goal of being the first to align three symbols vertically, horizontally, or diagonally. The twist is that each player can only have three pieces on the board at maximum, with the oldest piece being replaced whenver a new piece is placed to add an additional strategic component to a simple game.

![Screenshot of game](img/game.png?raw=true "Title")

## Installation

To build this game from source, you need to have Rust and Cargo installed on your computer. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

### Cloning the Repository

1. First, clone the repository to your local machine:

    ```sh
    git clone https://github.com/rbnyng/cooler_tic_tac_toe
    ```
2. Navigate to the project directory:
    ```sh
    cd cooler_tic_tac_toe
    ```

### Running the Game

To compile and run the game, use the following command from the project directory:
    ```sh
    cargo run
    ```

Or you can build it into an executable with:
    ```sh
    cargo build --release
    ```

### WebAssembly Deployment

To deploy the game as a WebAssembly application and play it in a web browser, use the following commands:

1. Install `trunk`:

    ```sh
    cargo install trunk
    ```

2. Install the required wasm target with:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

#### Web Local Testing

1. Build and serve the game locally on `http://127.0.0.1:8080` with:
    ```sh
    trunk serve
    ```

#### Web Deploy

1. Build the dist with:
    ```sh
    trunk build --release --public-url .
    ```

This generates a `dist` folder as the static HTML to deploy.

Alternatively, a workflow is included to automatically build and deploy to GitHub Pages. This is the version I used to deploy it [here](https://yourusername.github.io/rust_tic_tac_toe) 

## Usage

Upon starting the game, you will be presented with a 3x3 board. The game starts with player X. Players take turns placing their symbol on the board by clicking an empty cell. 

## License

This project is open source and available under the [MIT License](LICENSE).
