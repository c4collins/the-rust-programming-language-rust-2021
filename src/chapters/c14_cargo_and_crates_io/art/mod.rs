//! Art
//!
//! A library for making artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colours accorfing to the RYB color model.
    #[derive(Debug, Copy, Clone)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::{PrimaryColor, SecondaryColor};

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, PrimaryColor> {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Red) => Ok(SecondaryColor::Orange),
            (PrimaryColor::Blue, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Blue) => Ok(SecondaryColor::Green),
            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
                Ok(SecondaryColor::Purple)
            }
            (PrimaryColor::Blue, PrimaryColor::Blue) => Err(PrimaryColor::Blue),
            (PrimaryColor::Red, PrimaryColor::Red) => Err(PrimaryColor::Blue),
            (PrimaryColor::Yellow, PrimaryColor::Yellow) => Err(PrimaryColor::Blue),
        }
    }
}
