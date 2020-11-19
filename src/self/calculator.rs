// First we need to import or "use" io
use std::io;
pub fn calculator() {
  // Keep guessing numbers until user says no to play again
  loop {
    let guess_1: i32;
    let guess_2: i32;
    // Instantiate new string?
    loop {
      let mut first_input = String::new();
      // Please input your number
      println!("Please input first number");
      // Recieve user input
      match io::stdin().read_line(&mut first_input) {
        Ok(_) => (),
        Err(_) => (),
      }
      guess_1 = match first_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Not working");
          continue;
        }
      };
      break;
      // guess_1 = match first_input.trim().parse() {
      //   Ok(num) => num,
      //   Err(_) => println!("{} isn't a number!", first_input),
      // }
    }
    println!("First number was {}", guess_1);
    loop {
      let mut second_input = String::new();
      // Please input your number
      println!("Please input second number");
      // Recieve user input
      match io::stdin().read_line(&mut second_input) {
        Ok(_) => (),
        Err(_) => (),
      }
      guess_2 = match second_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Not working");
          continue;
        }
      };
      break;
      // guess_1 = match first_input.trim().parse() {
      //   Ok(num) => num,
      //   Err(_) => println!("{} isn't a number!", first_input),
      // }
    }
    println!("Second number was {}", guess_2);
    println!("{} plus {} is {}!", guess_1, guess_2, guess_1 + guess_2);
    println!("Would you like to play again?");
    let mut yes_no = String::new();
    match io::stdin().read_line(&mut yes_no) {
      Ok(_) => (),
      Err(_) => (),
    };
    match yes_no.to_string().as_str() {
      "yes" => continue,
      "no" => break,
      _ => {
        println!("RIP");
        return;
      }
    }
  }
}
