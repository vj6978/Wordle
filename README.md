#  Wordle 🧩

A sleek, terminal-based Wordle clone written in Rust. Test your vocabulary and logic skills directly from your command line!

![Rust](https://img.shields.io/badge/language-rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

-   **Random Word Generation**: Every game features a different 5-letter English word.
-   **Terminal UI**: Clean and colorful display using the `colored` crate.
-   **Input Validation**: Ensures guesses are exactly 5 alphabetic characters.
-   **Classic Gameplay**: 6 attempts to guess the hidden word with intuitive feedback.

## 🎮 How to Play

1.  Guess the **WORDLE** in six tries.
2.  Each guess must be a **five-letter word**.
3.  After each guess, the color of the tiles will change to show how close your guess was to the word:
    -   🟩 **Green**: The letter is in the word and in the correct spot.
    -   🟨 **Yellow**: The letter is in the word but in the wrong spot.
    -   🟥 **Red**: The letter is not in the word in any spot.
4.  To quit the game, type `QUIT` when prompted for a guess.

## 🚀 Getting Started

### Prerequisites

-   [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.
-   [Docker](https://www.docker.com/get-started) (optional, if you want to run via Docker).

### Installation & Running

#### Using Cargo

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/Primus.git
    cd Primus
    ```
2.  Run the game:
    ```bash
    cargo run
    ```

#### Using Docker

1.  Build the Docker image:
    ```bash
    docker build -t primus-wordle .
    ```
2.  Run the game in an interactive container:
    ```bash
    docker run -it primus-wordle
    ```

## 🛠️ Built With

-   [Rust](https://www.rust-lang.org/) - The programming language used.
-   [colored](https://crates.io/crates/colored) - For terminal string coloring.
-   [random_word](https://crates.io/crates/random_word) - For fetching random English words.
-   [rand](https://crates.io/crates/rand) - For random number generation.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
Happy Guessing! 🎯
