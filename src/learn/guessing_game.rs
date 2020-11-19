use rand::Rng;
use std::cmp::Ordering;
/// Rust apparently includes "the prelude" into every single program, so there's some imports that we don't write here
// Instead of std::cout, we're pulling the io module from the standard library of rust?
use std::io;

/// Objects immutable by default, have to specify mut to make it mutable
/// Like how in c++ strings are from the std::library, so is the string a type provided by the standard library. ::means
pub fn guess() {
  let min = 1;
  let max = 101;
  let secret_number = rand::thread_rng().gen_range(min, max);
  println!("The secret number is : {}", secret_number);
  loop {
    println!("Input your guess.");
    let mut guess = String::new();
    // Speculation: mut means that guess can be changed, kinda like the opposite of const. String:: is like typescript typing but with double ::, and new() creates a new block in memory
    // Y'know, rust with the whole automatic memory management and everything

    // kinda like std:: from c++, instatiating a new stdin() call where we read stuff??
    // I think that the & means that we're binding guess to the result
    // But why do we need mut then?
    // Automatic exception hanlding is really cool tho $$
    // And unused checking
    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");
    println!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!(
          "Expected a positive number between {} and {}, but got {} instead!",
          min, max, guess
        );
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
