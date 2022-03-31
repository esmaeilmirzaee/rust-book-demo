//! # Art
//! A library for modeling artistic concepts
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        crate::kinds::SecondaryColor { red, yellow, SecondaryColor::Purple }
    }
}

/// Function add_one adds one to the number given.
/// # Example
/// ```rust
/// let arg = 5;
/// let result = add_one(arg);
/// assert_eq!(arg + 1, result);
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_one_eq() {
        let num = 6;
        let result = add_one(num);
        assert_eq!(num + 1, result);
    }
}
