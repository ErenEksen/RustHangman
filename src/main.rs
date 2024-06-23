use rand::Rng;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io;

const MAX_TRIES: u8 = 6;

fn main() {
    println!("Welcome to Hangman!");
    run_game();
}

fn run_game() {
    let word = import_and_choose_word();
    let mut guessed_letters = HashSet::new();
    let mut tries = 0;

    loop {
        print_word(&word, &guessed_letters);
        print_hangman(tries);

        if is_word_guessed(&word, &guessed_letters) {
            println!("Congratulations! You have guessed the word!");
            break;
        }

        if tries >= MAX_TRIES {
            println!("Game over! The word was: {}", word);
            break;
        }

        match get_guess(&guessed_letters) {
            Some(guess) => {
                guessed_letters.insert(guess);
                if !word.contains(guess) {
                    tries += 1;
                    println!("Incorrect guess!");
                }
            }
            None => continue,
        }
    }
}

fn import_and_choose_word() -> String {
    let content = read_to_string("./words.txt");

    match content {
        Ok(content) => {
            let all_words: Vec<String> = content.lines().map(|line| line.to_string()).collect();
            return all_words[rand::thread_rng().gen_range(1..all_words.len())].clone();
        }
        Err(_) => {
            File::create("words.txt").expect("Could not create file");
            return "Hangman".to_string();
        }
    }
}
fn print_word(word: &str, guessed_letters: &HashSet<char>) {
    for c in word.chars() {
        if guessed_letters.contains(&c.to_lowercase().next().unwrap()) {
            print!("{} ", c);
        } else {
            print!("_ ");
        }
    }
    println!();
}

fn is_word_guessed(word: &str, guessed_letters: &HashSet<char>) -> bool {
    word.to_lowercase()
        .chars()
        .all(|c| guessed_letters.contains(&c))
}

fn get_guess(guessed_letters: &HashSet<char>) -> Option<char> {
    println!("Enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim().to_lowercase().chars().next()?;

    if !guess.is_ascii_alphabetic() {
        println!("Please enter a valid letter.");
        return None;
    }

    if guessed_letters.contains(&guess) {
        println!("You've already guessed that letter.");
        return None;
    }

    Some(guess)
}

fn print_hangman(tries: u8) {
    let hangman = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    ];

    println!("{}", hangman[tries as usize]);
}
