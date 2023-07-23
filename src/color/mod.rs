pub mod colorful;
use std::fmt::Display;


use crate::color::colorful::Color;


const RESET:&str = "\x1b[0m";


pub trait ColorString {
    fn fg(&self, color: Color) -> String;
    fn bg(&self, color: Color) -> String;
    fn set_color(&self, fg: Color, bg: Color) -> String;
}

impl<T: Display> ColorString for T {
    fn fg(&self, color: Color) -> String {
        format!("\x1b[38;5;{}m{}{}", color as usize, self, RESET)
    }

    fn bg(&self, color: Color) -> String {
        format!("\x1b[48;5;{}m{}{}", color as usize, self, RESET)
    }

    fn set_color(&self, fg: Color, bg: Color) -> String {
        format!("\x1b[38;5;{}m\x1b[48;5;{}m{}{}", fg as usize, bg as usize, self, RESET)
    }
}