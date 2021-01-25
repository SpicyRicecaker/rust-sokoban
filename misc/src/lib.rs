//! Use `//!` to denote comments describing the crate itself.
//! You usually add this to the beginning of the file, say `src/lib.rs`

// Pub use basically makes it possible for people that want to call your code to just go
// crate::desired_thing instead of having to figure out the structure of your code
pub use self::chest::slot_one::PrimaryColor;
pub use self::chest::slot_one::SecondaryColor;
pub use self::utils::mix;

/// The main function
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = misc::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Panics
/// Panics when ..., so don't call in these situations
///
/// # Errors
/// Errors when ..., returns a ... message based on that error
///
/// # Safety
/// It's unsafe when ... so make sure you only put ... into the function
///
/// ```
/// println!("This will actually be run in a test wutt");
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Used to store more misc stuff
pub mod chest {
    /// First slot
    pub mod slot_one {
        /// RYB primary colors
        pub enum PrimaryColor {
            Red,
            Yello,
            Blue
        }
        
        /// RYB secondary colors
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple
        }
        
    }
}

pub mod utils {
    use crate::chest::slot_one::*;
    
    /// Creates two primary colors in equal amounts to create
    /// A secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}