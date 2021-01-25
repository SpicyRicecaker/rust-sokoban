use std::{collections::HashMap, hash::Hash, thread};
use std::time::Duration;
// use HashMap;

fn main() {
    let user_specified_value = 26;
    let simulated_random_number = 7;

    generate_workout(user_specified_value, simulated_random_number);
}

// Struct that holds the execution of the closure
// Execute only if we need the resulting value
// Resulting value will also be cached
// Memoization, lazy evaluation
struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq,
{
    calculation: T,
    // value: Option<U>,
    values: HashMap<U, U>
}
impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    // Constructor, returns new self with
    // Some calculation and
    // `None` initial value
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    // When we call this, we want to check
    // if we already have something in the cache,
    // Otherwise get something in the cache
    fn value(&mut self, arg: U) -> U {
        // Final solution using `entry` with `or_insert`
        *self.values.entry(arg).or_insert((self.calculation)(arg))
        // Second solution using hashmap but still matching
        // match self.values.get(&arg) {
        //     Some(v) => *v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.values.insert(arg, v);
        //         v
        //     }
        // }

        // First solution with no hashmap
        // if let Some(i) = self.value {
        //     i
        // } else {
        //     let i = (self.calculation)(arg);
        //     self.value = Some(i);
        //     i
        // }
    }

    // fn value_def(&mut self, arg: u32) -> u32 {
    //     match self.value {
    //         Some(v) => v,
    //         None => {
    //             let i = (self.calculation)(arg);
    //             self.value = Some(i);
    //             i
    //         }
    //     }
    // }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // fn bob () {
    //     println!("hello");
    // }

    // Types are inferred for closures when they're used,
    // but if you don't call them the types can't be inferred
    // and the program won't compile
    // Similarily, if you call the same closure with different types
    // that also won't compile

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));

        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

// fn simulated_expensive_calculation(intensity: u32) -> &'static str {
//     "123"
// }

#[test]
fn call_with_different_values() {
    let mut cache = Cacher::new(|a| a);

    let _v1 = cache.value(1);
    let v2 = cache.value(2);

    assert_eq!(2, v2);
}
