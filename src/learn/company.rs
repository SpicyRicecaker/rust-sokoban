use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
}

impl Employee {
    // Get valid user array
    fn new_from_input() -> Employee {
        // Initiate place to store input
        let mut user_input = String::new();
        // Until parse int is true
        loop {
            // Get user input
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
            // Parse it
            let pattern = match Regex::new(r"(?i)add\s+(\S+)\s+to\s+(\S+)") {
                Ok(res) => res,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
            // Then match the input
            let name_apartment = match pattern.captures(&user_input) {
                Some(capture) => capture,
                None => {
                    println!(
                        "'{}' is not valid input, expected 'add [name] to [department]'",
                        user_input
                    );
                    continue;
                }
            };
            return Employee {
                name: String::from(&name_apartment[1]),
                department: String::from(&name_apartment[2]),
            };
        }
    }
}

pub fn company() {
    // Create index of department to employee
    let d_index:HashMap<String, Employee> = HashMap::new();
    // First find way to get key terms from user input
    let employee = Employee::new_from_input();
}
