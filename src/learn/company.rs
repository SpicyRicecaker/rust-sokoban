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
    fn add_to_index(&self, index: &mut HashMap<String, String>) {
        // Don't know how lifetimes work with references so for now we'll have to
        // insert by value
        // original hashmap was <&String, &Employee>
        index.insert(self.department.clone(), self.name.clone());
    }
}

pub fn company() {
    let mut running = true;
    // List all people in department sorted alphabetically
    // List all departments sorted alphabetically
    println!("Press ADD to add employee, DEPARTMENT to list people in department, COMPANY to list people in company, and QUIT to leave");
    let mut input = String::new();
    // Create index of department to employee
    let mut d_index: HashMap<String, String> = HashMap::new();
    while running {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        match input {
            "ADD" => {
                // First find way to get key terms from user input
                let employee = Employee::new_from_input();
                employee.add_to_index(&mut d_index);
            }
            "DEPARTMENT" => {
                
            }
            "COMPANY" => {}
            "QUIT" => running = !running,
        }
    }
}
