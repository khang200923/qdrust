use std::fmt;
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ColorMode {
    Random,
    White,
    Black,
}

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorMode::Random => write!(f, "random"),
            ColorMode::White => write!(f, "white"),
            ColorMode::Black => write!(f, "black"),
        }
    }
}