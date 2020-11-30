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
        println!("Please enter '[name] to [department]'");
        loop {
            // Initiate place to store input
            let mut user_input = String::new();
            // Until parse int is true
            // Get user input
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
            // Parse it
            let pattern = match Regex::new(r"(?i)(\S+)\s+to\s+(\S+)") {
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
                        "'{}' is not valid input, expected '[name] to [department]'",
                        user_input.trim()
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

struct Index {
    map: HashMap<String, Vec<String>>,
}

impl Index {
    fn new() -> Index {
        Index {
            map: HashMap::new(),
        }
    }
    fn getMap(&self) -> &HashMap<String, Vec<String>> {
        &self.map
    }
    fn add_to_index(&mut self, employee: &Employee) {
        // Don't know how lifetimes work with references so for now we'll have to
        // insert by value
        // original hashmap was <&String, &Employee>
        let employees = self
            .map
            .entry(employee.department.clone())
            .or_insert(Vec::new());
        employees.push(employee.name.clone());
        // Make sure it's alphabetical!
        employees.sort();
    }
    fn input() -> String {
        println!("Please enter the name of the department");
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => return input,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
        }
    }
    fn get_department_input(&self) -> Option<&Vec<String>> {
        self.get_department(Index::input())
    }
    fn get_department(&self, department: String) -> Option<&Vec<String>> {
        // Returns the vector
        self.map.get(&String::from(department.trim()))
    }
}

pub fn company() {
    let mut running = true;
    // List all people in department sorted alphabetically
    // List all departments sorted alphabetically
    println!("Press ADD to add employee, DEPARTMENT to list people in department, COMPANY to list people in company, and QUIT to leave.");
    // Create index of department to employee
    let mut department_index = Index::new();
    while running {
        let mut input = String::new();
        println!("Please enter a command\n");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        input = input.to_ascii_lowercase();
        let input = input.trim();
        match input {
            "add" => {
                // First find way to get key terms from user input
                let employee = Employee::new_from_input();
                department_index.add_to_index(&employee);
                println!("Successfully added employee.");
            }
            "department" => match department_index.get_department_input() {
                Some(vector) => {
                    println!("\n--employees--\n");
                    for employee in vector.iter() {
                        println!("{}", employee);
                    }
                }
                None => println!("No department found"),
            },
            "company" => {
                for (department, employees) in department_index.getMap() {
                    println!("{}   {}", employees[0], department);
                    for employee in employees.iter().skip(1) {
                        println!("{}", employee);
                    }
                }
            }
            "help" => {
                println!("Press ADD to add employee, DEPARTMENT to list people in department, COMPANY to list people in company, and QUIT to leave");
            }
            "quit" => running = !running,
            _ => println!(
                "'{}' is not a known command. Type HELP for commands.",
                input
            ),
        }
        println!("\n\n\n---------------------------\n")
    }
}
