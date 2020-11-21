use std::io;
// Generate nth fibonacci number based off of user input
pub fn generate_nth() {
  // Next try to parse it
  let n: i32 = get_valid_user_number("Please enter the nth number of fib");
  fibonacci(n);
}

// Gets valid user input in a loop, returns it
fn get_valid_user_number(prompt: &str) -> i32 {
  println!("{}", prompt);
  // Initiate place to store input
  let mut user_input = String::new();
  // Until parse int is true
  loop {
    // Get user input
    match io::stdin().read_line(&mut user_input) {
      Ok(_) => {}
      Err(_) => {
        println!("You broke input");
        continue;
      }
    }
    // Next parse it
    match user_input.trim().parse() {
      Ok(num) => return num,
      Err(_) => println!("{} is not a number! Please try again", user_input),
    }
  }
}

// Given n number, print fib with iteration
fn fibonacci(n: i32) {
  let mut first = 0;
  let mut second = 1;
  if n == 0 {
    println!("GG Well played");
    return;
  }
  for _i in 0..n {
    println!("{}", first);
    let t = second;
    second += first;
    first = t;
  }
}