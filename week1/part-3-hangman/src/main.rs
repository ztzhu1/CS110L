// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 10;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0.. words.len())].trim())
}

fn print_word(secret_word_chars: &Vec<char>, mask: &Vec<bool>) {
    assert_eq!(secret_word_chars.len(), mask.len());
    print!("The word so far is ");
    for i in 0..mask.len() {
        if mask[i] {
            print!("{}", secret_word_chars[i]);
        } else {
            print!("-");
        }
    }
    print!("\n");
}

fn print_guessed(guessed: &Vec<char>) {
    print!("You have guessed the following letters: ");
    for c in guessed {
        print!("{}", *c);
    }
    print!("\n");
}

fn print_chance(chance: u32) {
    println!("You have {} guesses left", chance);
}

fn get_a_letter(secret_word_chars: &Vec<char>, mask: &mut Vec<bool>, guessed: &mut Vec<char>, chance: u32) {
    print!("Please guess a letter: ");
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
    guessed.push(guess.chars().nth(0).unwrap());
    let mut found = false;
    for i in 0..mask.len() {
        if secret_word_chars[i] ==  guessed[(NUM_INCORRECT_GUESSES - chance) as usize] {
            mask[i] = true;
            found = true;
        }
    }
    if !found {
        println!("Sorry, that letter is not in the word");
    }
    print!("\n");
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)

    let mut mask: Vec<bool> = Vec::new();
    for _ in 0..secret_word_chars.len() {
        mask.push(false)
    }
    let mut chance = NUM_INCORRECT_GUESSES;
    let mut guessed: Vec<char> = Vec::new();
    while chance != 0 {
        print_word(&secret_word_chars, &mask);
        print_guessed(&guessed);
        print_chance(chance);
        get_a_letter(&secret_word_chars, &mut mask, &mut guessed, chance);
        if mask.iter().all(|x| *x) {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }

        chance -= 1;
        if chance == 0 {
            println!("Sorry, you ran out of guesses!");
        }
    }
}
