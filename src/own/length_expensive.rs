use std::time::SystemTime;
pub fn length_expensive() {
  index();
  length();
}

fn took(msg: &str, start: SystemTime) {
    match start.elapsed() {
        Ok(elapsed) => {
            println!("{} took {} ms", msg, elapsed.as_millis())
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}

fn index() {
    let create_start_index = SystemTime::now();
    // Vector with index
    let mut nums: Vec<i32> = Vec::new();
    let mut index = 0;
    for i in 0..1000000000 {
        nums.push(i);
        index += 1;
    }
    took("index add", create_start_index);
    println!("length is {}", index);
    took("index length", create_start_index);
    
}

fn length() {
    let create_start_length = SystemTime::now();
    // Instantiate new VECTOR with 100 variables
    let mut nums: Vec<i32> = Vec::new();
    for i in 0..1000000000 {
        nums.push(i);
    }
    took("length add", create_start_length);
    println!("length is {}", nums.len());
    took("length add", create_start_length);
}
