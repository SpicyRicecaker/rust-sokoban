use std::io;

pub fn test() {
    // take in string

    let input = get_valid_user_number("Input something bro");

    // next pass it in to the length recognize func
    let first_word = get_second_word(&input);
    println!("The last word of '{}' is '{}'", input, first_word);
}

// Gets valid user input in a loop, returns it, trims string
fn get_valid_user_number(prompt: &str) -> String {
    println!("{}", prompt);
    // Initiate place to store input
    let mut user_input = String::new();
    // Until parse int is true
    // Get user input
    io::stdin().read_line(&mut user_input).expect("ff");
    String::from(user_input.trim())
}

// Gets the first word in a string
fn get_first_word(input: &str) -> &str {
    let s = input.as_bytes();

    // If space found return the word
    for (i, &character) in s.iter().enumerate() {
        if character == b' ' {
            return &input[..i];
        }
    }

    // Otherwise just return the input
    input
}

// Gets all words in a string
// We don't know the split function syntax so might as well
// Fk. Need vectors lol, can't type a dynamic array as a return.
// We'll try last word then
fn get_last_word(input: &str) -> &str {
    // First we need to get the byte array of the string so we can actually iterate over it
    let byte = input.as_bytes();

    for (i, &byte) in byte.iter().enumerate().rev() {
        if byte == b' ' {
            return &input[i..];
        }
    }
    input
}

// Get second word in string
fn get_second_word(input: &str) -> &str {
    let byte = input.as_bytes();

    let mut first: usize = 0;
    for (i, &byte) in byte.iter().enumerate() {
        if byte == b' ' {
            if first != 0 {
                println!("return bye bye");
                return &input[first + 1..i];
            } else {
                println!("setting {} to {}", first, i);
                first = i;
            }
        }
    }

    if first != 0 {
        println!("{} is not 0", first);
        println!("Returning {}", &input[first..]);
        &input[first+1..]
    } else {
        input
    }
}
