use std::io;
pub fn f_to_c() {
  let mut user_input = String::new();
  println!("Please enter a fahrenheit temperature!");
  match io::stdin().read_line(&mut user_input) {
    Ok(_) => {}
    Err(_) => println!("BAD"),
  }
  let parsed: f64 = match user_input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Not a valid temp lol");
      return;
    }
  };
  println!(
    "{} fahrenheit in celcius is {}",
    parsed,
    (parsed - 32.0) * 5.0 / 9.0
  );
}
