use regex::Regex;
use std::collections::HashMap;
use std::io;
struct Dataset {
    list: Vec<i32>,
}

impl Dataset {
    // fn set_list(&mut self, list: Vec<i32>) {
    //     self.list = list;
    // }
    // Takes in vector and returns the corresponding number!
    fn get_mean(&self) -> f64 {
        // Loop
        let mut total = 0;
        let mut length = 0;
        for num in self.list.iter() {
            total += num;
            length += 1;
        }
        total as f64 / length as f64
    }
    fn get_median(&mut self) -> f64 {
        self.sort();
        let len = self.list.len();

        if len == 0 {
            0.0
        } else if len % 2 == 0 {
            // Else if even, take the average of (length/2) and (length/2-1)
            (self.list[(len / 2)] as f64 + self.list[(len / 2 - 1)] as f64) / 2.0
        } else {
            // Then get the length. If odd, return (length - 1)/2
            self.list[(len - 1) / 2] as f64
        }
    }

    fn get_mode(&self) -> i32 {
        let mut mode_map: HashMap<i32, i32> = HashMap::new();
        for &num in self.list.iter() {
            let appearance = mode_map.entry(num).or_insert(0);
            *appearance += 1
        }
        let mut mode: i32 = 0;
        let mut high_appearance: i32 = 0;
        for (key, value) in mode_map {
            if value > high_appearance {
                high_appearance = value;
                mode = key;
            }
        }
        mode
    }

    fn sort(&mut self) {
        // Vectors in rust jankkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk
        // let mut temp: Vec<i32> = Vec::new();
        // for &num in self.list.iter() {
        //     temp.push(num);
        // }

        // for (i, capacity) in temp.into_iter().enumerate() {
        //     for (j, bubble) in temp.into_iter().enumerate().skip(i) {
        //         println!(
        //             "Comparing the bubbling {} and capacity {}",
        //             bubble, capacity
        //         );
        //         if bubble < capacity {
        //             temp.swap(i as usize, j as usize);
        //         }
        //     }
        // }

        // self.list = temp;
        self.list.sort();
    }
}

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn avg_mean_mode() {
    println!("Please enter a list of comma separated numbers");
    // Get user input of num list and turn it into a vector
    let mut dataset1 = Dataset {
        list: get_valid_user_number_list(),
    };
    // Function that finds mean, median, and range
    let data = (
        dataset1.get_mean(),
        dataset1.get_median(),
        dataset1.get_mode(),
    );
    println!(
        "mean is {}, median is {}, mode is {}",
        data.0, data.1, data.2
    );
}

// Takes in a string slice (of comma separated numbers)
// and returns a vector of str ( i think )

enum SplitResult {
    List(Vec<i32>),
    Err,
}

fn split_by_comma(input: &str) -> SplitResult {
    // Create new regex
    let regex = Regex::new(r",\s*").unwrap();
    let sentence = regex.split(input);

    let mut list: Vec<i32> = Vec::new();
    for word in sentence {
        let i: i32 = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => return SplitResult::Err,
        };
        list.push(i);
    }
    SplitResult::List(list)
}

// Get valid user array
fn get_valid_user_number_list() -> Vec<i32> {
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
        // Make it into an array
        // let number_as_string_array = split_by_comma(user_input);
        match split_by_comma(&user_input[..]) {
            SplitResult::List(vec_boi) => return vec_boi,
            SplitResult::Err => {
                println!("Unrecognized characters found, please re-enter the list!")
            }
        }
    }
}
