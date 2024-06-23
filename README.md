# 🕹️ Hangman Game in Rust

Welcome to the Hangman game written in Rust! This project is a command-line version of the classic word-guessing game.

## 📑 Table of Contents

- [✨ Features](#-features)
- [📦 Installation](#-installation)
- [🛠️ How to Build](#-how-to-build)
- [📝 Setting Up the Words File](#-setting-up-the-words-file)
- [🎮 Usage](#-usage)

## ✨ Features

- Play the classic Hangman game from the command line.
- Read words from a text file (`words.txt`).
- Keep track of your guesses and remaining attempts.

## 📦 Installation

To get started with the Hangman game, you'll need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## 🛠️ How to Build

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/hangman-rust.git
    cd hangman-rust
    ```

2. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

3. The compiled binary will be located in the `target/release` directory.

## 📝 Setting Up the Words File

The game requires a `words.txt` file that contains the words to be used in the game. Each word should be on a new line.

1. Create a `words.txt` file in the root directory of the project:

    ```bash
    touch words.txt
    ```

2. Open `words.txt` in a text editor and add your words, one per line:

    ```plaintext
    Rust
    Hangman
    Juicy Apple
    Php is dead
    Jomo
    Why I am
    Writing random
    Words here
    I do not even
    Know
    ```

3. Save and close the file.

## 🎮 Usage

To start the game, run the compiled binary:

```bash
./target/release/HangmanRust
```
