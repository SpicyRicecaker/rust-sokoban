use regex::Regex;
use std::io;

// enum SplitResult {
//     List(Vec<String>),
//     Err,
// }

// input as String
// split input by spaces (gives &str)
// 1
// word as &str gets converted into String, passed to_pig_latin, and pushed into vector
// cons = have to iterate over the entire vector again to print it
// pros = no concatenation, we won't have to mess with spaces around words, more modular, you can modify input
// word as &str gets converted into String, passed to_pig_latin, and appended to String
//
// always format!("{} {}", string1, string2)

use std::time::SystemTime;

fn took(msg: &str, start: SystemTime) {
    match start.elapsed() {
        Ok(elapsed) => {
            println!("{} took {} ms", msg, elapsed.as_micros())
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}

const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

pub fn pig_latin() {
    println!("Please enter words split by space");
    let input = input();
    let result = split_by_space(input);
    println!("{}", result);
}

// Get valid user array
fn input() -> String {
    // Initiate place to store input
    let mut user_input = String::new();
    // Until parse int is true
    loop {
        // Get user input
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => return user_input,
            Err(_) => {
                println!("You broke input");
                continue;
            }
        }
    }
}

// Takes in &str, returns &str
fn split_by_space(input: String) -> String {
    // Create new regex
    let regex = Regex::new(r"\s+").unwrap();
    let sentence = regex.split(&input.trim());

    let mut new: String = String::new();
    for word in sentence {
        new = format!("{} {}", new, &to_pig_latin(word));
    }
    new
}

// Convert to pig latin
// If consonant, find cluster, move to back, add -ay
// If vowel, just add "yay" or "ay" to the end of the word
fn to_pig_latin(word: &str) -> String {
    if word.len() != 0 {
        // First check if first is a vowel
        for (i, character) in word.bytes().enumerate() {
            if VOWELS.iter().any(|&x| x == character.to_ascii_lowercase()) {
                if i == 0 {
                    return String::from(word) + "yay";
                }
                return format!("{}{}ay", &word[i..], &word[..i]);
            }
        }
        return format!("{}{}ay", &word[1..], &word[..1]);
    } else {
        String::from(word)
    }
}